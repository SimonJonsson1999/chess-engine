use super::EngineConfig;
use chess_engine::board::{Board, Color, GameState};
use chess_engine::player::AI;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const GAME_COUNT: usize = 100;
const MAX_PLIES: usize = 400;
const RESULTS_DIRECTORY: &str = "benchmark-results";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    WhiteWin,
    Draw,
    BlackWin,
}

impl Outcome {
    fn name(self) -> &'static str {
        match self {
            Self::WhiteWin => "white_win",
            Self::Draw => "draw",
            Self::BlackWin => "black_win",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Termination {
    Checkmate,
    Stalemate,
    Repetition,
    FiftyMoveRule,
    MoveLimit,
}

impl Termination {
    fn name(self) -> &'static str {
        match self {
            Self::Checkmate => "checkmate",
            Self::Stalemate => "stalemate",
            Self::Repetition => "repetition",
            Self::FiftyMoveRule => "fifty_move_rule",
            Self::MoveLimit => "move_limit",
        }
    }
}

struct GameResult {
    outcome: Outcome,
    termination: Termination,
    plies: usize,
    elapsed: Duration,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct EngineScore {
    wins: usize,
    draws: usize,
    losses: usize,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Scores {
    white_wins: usize,
    draws: usize,
    black_wins: usize,
    engines: [EngineScore; 2],
}

impl Scores {
    fn record(&mut self, outcome: Outcome, white_engine: usize) {
        let black_engine = 1 - white_engine;
        match outcome {
            Outcome::WhiteWin => {
                self.white_wins += 1;
                self.engines[white_engine].wins += 1;
                self.engines[black_engine].losses += 1;
            }
            Outcome::Draw => {
                self.draws += 1;
                for engine in &mut self.engines {
                    engine.draws += 1;
                }
            }
            Outcome::BlackWin => {
                self.black_wins += 1;
                self.engines[black_engine].wins += 1;
                self.engines[white_engine].losses += 1;
            }
        }
    }
}

fn play_game(board: &mut Board, players: &mut [Box<dyn AI>; 2], max_plies: usize) -> GameResult {
    let start = Instant::now();
    let mut plies = 0;
    loop {
        // Natural endings take precedence, including on the final allowed ply.
        let ending = match board.game_state() {
            GameState::Checkmate => Some((
                match board.turn {
                    Color::White => Outcome::BlackWin,
                    Color::Black => Outcome::WhiteWin,
                },
                Termination::Checkmate,
            )),
            GameState::Stalemate => Some((Outcome::Draw, Termination::Stalemate)),
            GameState::DrawRepetition => Some((Outcome::Draw, Termination::Repetition)),
            GameState::FiftyMoveRule => Some((Outcome::Draw, Termination::FiftyMoveRule)),
            GameState::Ongoing | GameState::Check => None,
        };
        if let Some((outcome, termination)) = ending {
            return GameResult {
                outcome,
                termination,
                plies,
                elapsed: start.elapsed(),
            };
        }
        if plies >= max_plies {
            return GameResult {
                outcome: Outcome::Draw,
                termination: Termination::MoveLimit,
                plies,
                elapsed: start.elapsed(),
            };
        }

        let mv = players[board.turn.idx()].choose_move(board);
        board.move_piece(mv);
        plies += 1;
    }
}

fn csv_field(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn run_matches(
    configs: [EngineConfig; 2],
    game_count: usize,
    csv: &mut impl Write,
    output: &mut impl Write,
    mut play: impl FnMut(&mut Board, &mut [Box<dyn AI>; 2]) -> GameResult,
) -> io::Result<Scores> {
    writeln!(
        csv,
        "game,white_engine,black_engine,result,termination,plies,elapsed_ms,white_wins,draws,black_wins"
    )?;
    csv.flush()?;
    let mut scores = Scores::default();

    for game in 0..game_count {
        let white_engine = game % 2;
        let black_engine = 1 - white_engine;
        // Factories are called for every game so no search cache is carried over.
        let mut players = [
            (configs[white_engine].create)(),
            (configs[black_engine].create)(),
        ];
        let result = play(&mut Board::default(), &mut players);
        scores.record(result.outcome, white_engine);
        writeln!(
            csv,
            "{},{},{},{},{},{},{},{},{},{}",
            game + 1,
            csv_field(configs[white_engine].name),
            csv_field(configs[black_engine].name),
            result.outcome.name(),
            result.termination.name(),
            result.plies,
            result.elapsed.as_millis(),
            scores.white_wins,
            scores.draws,
            scores.black_wins,
        )?;
        csv.flush()?;

        writeln!(
            output,
            "Game {}/{} | White: {} | Black: {} | {} ({}) | {} plies, {:.2}s | White/draw/Black: {}/{}/{}",
            game + 1,
            game_count,
            configs[white_engine].name,
            configs[black_engine].name,
            result.outcome.name(),
            result.termination.name(),
            result.plies,
            result.elapsed.as_secs_f64(),
            scores.white_wins,
            scores.draws,
            scores.black_wins,
        )?;
        output.flush()?;
    }

    writeln!(output, "\nCompleted {game_count} games")?;
    writeln!(
        output,
        "White wins: {} | Draws: {} | Black wins: {}",
        scores.white_wins, scores.draws, scores.black_wins,
    )?;
    for (config, score) in configs.iter().zip(&scores.engines) {
        writeln!(
            output,
            "{} | Wins: {} | Draws: {} | Losses: {}",
            config.name, score.wins, score.draws, score.losses,
        )?;
    }
    output.flush()?;
    Ok(scores)
}

fn create_results_file(directory: &Path) -> io::Result<(PathBuf, File)> {
    fs::create_dir_all(directory)?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(io::Error::other)?
        .as_millis();
    for suffix in 0_u64.. {
        let path = directory.join(format!("benchmark-{timestamp}-{suffix}.csv"));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    unreachable!("Results filename suffixes exhausted")
}

pub(super) fn run(configs: [EngineConfig; 2]) -> io::Result<()> {
    let (path, file) = create_results_file(Path::new(RESULTS_DIRECTORY))?;
    let mut csv = BufWriter::new(file);
    let mut output = io::stdout().lock();
    writeln!(
        output,
        "Benchmark: {GAME_COUNT} games, colours alternate, {MAX_PLIES}-ply limit"
    )?;
    writeln!(output, "Results: {}", path.display())?;
    output.flush()?;
    run_matches(
        configs,
        GAME_COUNT,
        &mut csv,
        &mut output,
        |board, players| play_game(board, players, MAX_PLIES),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess_engine::board::{MoveKind, PieceMove, Square};
    use chess_engine::evaluation::BoardEvaluation;
    use chess_engine::move_gen::MoveGenerator;
    use chess_engine::player::Computer;
    use chess_engine::search::NegaMaxAlphaBetaSearcher;
    use std::cell::Cell;

    struct NoMovesExpected;

    impl AI for NoMovesExpected {
        fn choose_move(&mut self, _: &mut Board) -> PieceMove {
            panic!("No move should be requested after the game ends");
        }
    }

    fn no_moves_expected() -> [Box<dyn AI>; 2] {
        [Box::new(NoMovesExpected), Box::new(NoMovesExpected)]
    }

    #[test]
    fn terminal_positions_assign_the_correct_winner_and_draw_reason() {
        for (fen, outcome, termination) in [
            (
                "7k/6Q1/5K2/8/8/8/8/8 b - - 0 1",
                Outcome::WhiteWin,
                Termination::Checkmate,
            ),
            (
                "8/8/8/8/8/5k2/6q1/7K w - - 0 1",
                Outcome::BlackWin,
                Termination::Checkmate,
            ),
            (
                "7k/5K2/6Q1/8/8/8/8/8 b - - 0 1",
                Outcome::Draw,
                Termination::Stalemate,
            ),
            (
                "4k3/8/8/8/8/8/8/4K3 w - - 100 1",
                Outcome::Draw,
                Termination::FiftyMoveRule,
            ),
        ] {
            let mut board = Board::from_fen(fen);
            // A zero-ply cap verifies that natural endings take precedence.
            let result = play_game(&mut board, &mut no_moves_expected(), 0);
            assert_eq!(result.outcome, outcome, "{fen}");
            assert_eq!(result.termination, termination, "{fen}");
            assert_eq!(result.plies, 0);
        }
    }

    struct KnightCycle {
        squares: [Square; 2],
        moves: usize,
    }

    impl AI for KnightCycle {
        fn choose_move(&mut self, _: &mut Board) -> PieceMove {
            let from = self.moves % 2;
            self.moves += 1;
            PieceMove::new(self.squares[from], self.squares[1 - from], MoveKind::Quiet)
        }
    }

    fn cycling_players() -> [Box<dyn AI>; 2] {
        [
            Box::new(KnightCycle {
                squares: [Square::G1, Square::F3],
                moves: 0,
            }),
            Box::new(KnightCycle {
                squares: [Square::G8, Square::F6],
                moves: 0,
            }),
        ]
    }

    #[test]
    fn repetition_on_the_last_allowed_ply_takes_precedence_over_the_limit() {
        let mut board = Board::default();
        let result = play_game(&mut board, &mut cycling_players(), 8);
        assert_eq!(result.outcome, Outcome::Draw);
        assert_eq!(result.termination, Termination::Repetition);
        assert_eq!(result.plies, 8);
    }

    #[test]
    fn an_ongoing_game_stops_at_the_ply_limit() {
        let mut board = Board::default();
        let result = play_game(&mut board, &mut cycling_players(), 3);
        assert_eq!(result.outcome, Outcome::Draw);
        assert_eq!(result.termination, Termination::MoveLimit);
        assert_eq!(result.plies, 3);
        assert_eq!(board.game_state(), GameState::Ongoing);
    }

    thread_local! {
        static FACTORY_CALLS: Cell<usize> = const { Cell::new(0) };
    }

    struct FreshPlayer {
        has_moved: bool,
    }

    impl AI for FreshPlayer {
        fn choose_move(&mut self, board: &mut Board) -> PieceMove {
            assert!(!self.has_moved, "Player state must not carry between games");
            self.has_moved = true;
            MoveGenerator::generate_valid_moves(board)
                .into_iter()
                .next()
                .unwrap()
        }
    }

    fn create_fresh_player() -> Box<dyn AI> {
        FACTORY_CALLS.with(|count| count.set(count.get() + 1));
        Box::new(FreshPlayer { has_moved: false })
    }

    #[derive(Default)]
    struct RecordingWriter {
        bytes: Vec<u8>,
        flushed: Vec<usize>,
    }

    impl Write for RecordingWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            self.flushed.push(self.bytes.len());
            Ok(())
        }
    }

    #[test]
    fn a_hundred_games_alternate_colours_reset_players_and_record_the_correct_scores() {
        FACTORY_CALLS.with(|count| count.set(0));
        let configs = [
            EngineConfig {
                name: "A",
                create: create_fresh_player,
            },
            EngineConfig {
                name: "B",
                create: create_fresh_player,
            },
        ];
        let mut csv = RecordingWriter::default();
        let mut output = Vec::new();
        let mut played = 0;
        let scores = run_matches(configs, 100, &mut csv, &mut output, |board, players| {
            assert_eq!(*board, Board::default(), "Every game needs a fresh board");
            for player in players {
                let mv = player.choose_move(board);
                board.move_piece(mv);
            }
            let outcome = [Outcome::WhiteWin, Outcome::Draw, Outcome::BlackWin][played % 3];
            played += 1;
            GameResult {
                outcome,
                termination: if outcome == Outcome::Draw {
                    Termination::Stalemate
                } else {
                    Termination::Checkmate
                },
                plies: 2,
                elapsed: Duration::from_millis(10),
            }
        })
        .unwrap();

        assert_eq!(played, 100);
        FACTORY_CALLS.with(|count| assert_eq!(count.get(), 200));
        assert_eq!(
            scores,
            Scores {
                white_wins: 34,
                draws: 33,
                black_wins: 33,
                engines: [
                    EngineScore {
                        wins: 33,
                        draws: 33,
                        losses: 34
                    },
                    EngineScore {
                        wins: 34,
                        draws: 33,
                        losses: 33
                    },
                ],
            }
        );
        let text = String::from_utf8(csv.bytes.clone()).unwrap();
        let lines: Vec<_> = text.lines().collect();
        assert_eq!(lines.len(), 101);
        let mut white_games = [0; 2];
        for (game, line) in lines[1..].iter().enumerate() {
            let (white, black) = if game % 2 == 0 {
                ("A", "B")
            } else {
                ("B", "A")
            };
            assert!(line.starts_with(&format!("{},\"{white}\",\"{black}\",", game + 1)));
            white_games[game % 2] += 1;
        }
        assert_eq!(white_games, [50, 50]);
        assert!(lines[100].ends_with(",34,33,33"));
        // The header and every complete row are flushed separately.
        assert_eq!(csv.flushed.len(), 101);
        for &length in &csv.flushed {
            assert_eq!(csv.bytes[length - 1], b'\n');
        }
        assert_eq!(csv.flushed.last(), Some(&csv.bytes.len()));
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("Completed 100 games"));
        assert!(output.contains("White wins: 34 | Draws: 33 | Black wins: 33"));
        assert!(output.contains("A | Wins: 33 | Draws: 33 | Losses: 34"));
        assert!(output.contains("B | Wins: 34 | Draws: 33 | Losses: 33"));
    }

    #[test]
    fn csv_quotes_engine_names() {
        assert_eq!(csv_field("A, \"custom\""), "\"A, \"\"custom\"\"\"");
    }

    #[test]
    fn a_csv_failure_stops_before_the_next_game() {
        struct FailingWriter;
        impl Write for FailingWriter {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                Ok(bytes.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Err(io::Error::other("Disk full"))
            }
        }
        let configs = [EngineConfig {
            name: "unused",
            create: || Box::new(NoMovesExpected),
        }; 2];
        let error = run_matches(configs, 100, &mut FailingWriter, &mut Vec::new(), |_, _| {
            panic!("No games should run when the results header cannot be flushed");
        })
        .unwrap_err();
        assert_eq!(error.to_string(), "Disk full");
    }

    #[test]
    fn results_files_are_unique_and_preserve_previous_content() {
        let directory = std::env::temp_dir().join(format!(
            "chess-benchmark-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        let (first_path, mut first) = create_results_file(&directory).unwrap();
        first.write_all(b"previous results").unwrap();
        first.flush().unwrap();
        drop(first);
        let (second_path, second) = create_results_file(&directory).unwrap();
        drop(second);
        assert_ne!(first_path, second_path);
        assert_eq!(fs::read(&first_path).unwrap(), b"previous results");
        fs::remove_file(first_path).unwrap();
        fs::remove_file(second_path).unwrap();
        fs::remove_dir(directory).unwrap();
    }

    #[test]
    fn real_searchers_can_play_a_short_headless_game() {
        let mut board = Board::default();
        let mut players: [Box<dyn AI>; 2] = std::array::from_fn(|_| {
            Box::new(Computer::new(
                NegaMaxAlphaBetaSearcher::new(BoardEvaluation),
                1,
            )) as Box<dyn AI>
        });
        let result = play_game(&mut board, &mut players, 4);
        assert_eq!(result.plies, 4);
        assert_eq!(result.outcome, Outcome::Draw);
        assert_eq!(result.termination, Termination::MoveLimit);
        assert_eq!(board.turn, Color::White);
        assert_eq!(board.full_move, 3);
    }
}
