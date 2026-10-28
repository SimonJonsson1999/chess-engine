use chess_engine::board::{Board, GameState, MoveKind, PieceMove, Square};
use chess_engine::engine::Engine;
use chess_engine::evaluation::Evaluator;
use chess_engine::move_gen::MoveGenerator;
use chess_engine::search::{
    MiniMaxAlphaBetaSercher, MiniMaxSearcher, NegaMaxAlphaBetaSearcher, Searcher,
};

const KNIGHT_CYCLE: [(Square, Square); 4] = [
    (Square::G1, Square::F3),
    (Square::G8, Square::F6),
    (Square::F3, Square::G1),
    (Square::F6, Square::G8),
];

fn play(board: &mut Board, moves: &[(Square, Square)]) {
    for &(from, to) in moves {
        let mv = MoveGenerator::generate_valid_moves(board)
            .into_iter()
            .find(|mv| mv.from == from && mv.to == to)
            .expect("Test move must be legal");
        board.move_piece(mv);
    }
}

#[test]
fn third_occurrence_counts_the_initial_position_and_undo_restores_history() {
    let mut board = Board::default();
    assert!(!board.is_threefold_repetition());
    play(&mut board, &KNIGHT_CYCLE);
    assert!(!board.is_threefold_repetition());
    assert_eq!(board.game_state(), GameState::Ongoing);
    play(&mut board, &KNIGHT_CYCLE);
    assert!(board.is_threefold_repetition());
    assert_eq!(board.game_state(), GameState::DrawRepetition);

    // Move generation must leave both the position and repetition history intact.
    MoveGenerator::generate_valid_moves(&mut board);
    assert_eq!(board.game_state(), GameState::DrawRepetition);
    board.undo();
    assert!(!board.is_threefold_repetition());
    play(&mut board, &KNIGHT_CYCLE[3..]);
    assert!(board.is_threefold_repetition());

    board.move_piece(PieceMove::new(
        Square::E2,
        Square::E4,
        MoveKind::DoublePawnPush,
    ));
    assert!(!board.is_threefold_repetition());
    board.undo();
    assert!(board.is_threefold_repetition());

    for _ in 0..8 {
        board.undo();
    }
    assert_eq!(board, Board::default());
}

#[test]
fn fen_load_starts_with_no_repetition_history() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 40 20");
    assert!(!board.is_threefold_repetition());
    play(&mut board, &KNIGHT_CYCLE);
    assert!(!board.is_threefold_repetition());
    play(&mut board, &KNIGHT_CYCLE);
    assert_eq!(board.game_state(), GameState::DrawRepetition);
}

#[test]
fn returning_a_rook_does_not_restore_castling_rights() {
    let mut board = Board::from_fen("4k1n1/8/8/8/8/8/8/4K2R w K - 0 1");
    let initial_hash = board.zobrist_hash;
    let rook_cycle = [
        (Square::H1, Square::H2),
        (Square::G8, Square::F6),
        (Square::H2, Square::H1),
        (Square::F6, Square::G8),
    ];
    play(&mut board, &rook_cycle);
    assert_ne!(initial_hash, board.zobrist_hash);
    play(&mut board, &rook_cycle);
    assert!(!board.is_threefold_repetition());
    play(&mut board, &rook_cycle);
    assert_eq!(board.game_state(), GameState::DrawRepetition);
}

#[test]
fn unavailable_en_passant_does_not_prevent_repetition() {
    for fen in [
        // No pawn can capture en passant.
        "4k1n1/8/8/3p4/8/8/8/4K1N1 w - d6 0 1",
        // The capturing pawn is pinned along the e-file.
        "k3r1n1/8/8/3pP3/8/8/8/4K1N1 w - d6 0 1",
        // Removing both pawns would expose the king to a rook along rank five.
        "6nk/8/8/K4pPr/8/8/8/6N1 w - f6 0 1",
    ] {
        let mut board = Board::from_fen(fen);
        let initial_hash = board.zobrist_hash;
        play(&mut board, &KNIGHT_CYCLE);
        assert_eq!(initial_hash, board.zobrist_hash, "{fen}");
        play(&mut board, &KNIGHT_CYCLE);
        assert_eq!(board.game_state(), GameState::DrawRepetition, "{fen}");
    }
}

#[test]
fn available_en_passant_changes_position_identity() {
    let mut board = Board::from_fen("4k1n1/8/8/3pP3/8/8/8/4K1N1 w - d6 0 1");
    let initial_hash = board.zobrist_hash;
    play(&mut board, &KNIGHT_CYCLE);
    assert_ne!(initial_hash, board.zobrist_hash);
    play(&mut board, &KNIGHT_CYCLE);
    assert!(!board.is_threefold_repetition());
    play(&mut board, &KNIGHT_CYCLE);
    assert_eq!(board.game_state(), GameState::DrawRepetition);
}

#[test]
fn human_moves_stop_after_repetition_draw() {
    let mut engine = Engine::default();
    for _ in 0..2 {
        for (from, to) in KNIGHT_CYCLE {
            assert!(engine.make_move(from, to));
        }
    }
    assert_eq!(engine.game_state, GameState::DrawRepetition);
    assert!(engine.is_game_over());
    let hash = engine.board.zobrist_hash;
    assert!(!engine.make_move(Square::E2, Square::E4));
    engine.update();
    assert_eq!(engine.board.zobrist_hash, hash);
    assert_eq!(engine.game_state, GameState::DrawRepetition);
}

struct LosingEvaluation;

impl Evaluator for LosingEvaluation {
    fn evaluate(&self, _board: &Board) -> i32 {
        -1_000
    }
}

fn assert_searcher_chooses_repetition_draw(searcher: &mut impl Searcher) {
    // Black starts this cycle, leaving White to complete the third occurrence.
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
    let mut board = Board::from_fen(fen);
    let black_first_cycle = [
        KNIGHT_CYCLE[1],
        KNIGHT_CYCLE[0],
        KNIGHT_CYCLE[3],
        KNIGHT_CYCLE[2],
    ];
    play(&mut board, &black_first_cycle);
    play(&mut board, &black_first_cycle[..3]);
    assert!(!board.is_threefold_repetition());

    let best_move = searcher.best_move(&mut board, 1);
    assert_eq!((best_move.from, best_move.to), (Square::F3, Square::G1));
    let mut expected = Board::from_fen(fen);
    play(&mut expected, &black_first_cycle);
    play(&mut expected, &black_first_cycle[..3]);
    assert_eq!(
        board, expected,
        "Search must restore the entire board and history"
    );
    board.move_piece(best_move);
    assert_eq!(board.game_state(), GameState::DrawRepetition);
}

#[test]
fn negamax_chooses_repetition_draw_at_the_search_horizon() {
    assert_searcher_chooses_repetition_draw(&mut NegaMaxAlphaBetaSearcher::new(LosingEvaluation));
}

#[test]
fn minimax_chooses_repetition_draw_at_the_search_horizon() {
    assert_searcher_chooses_repetition_draw(&mut MiniMaxSearcher::new(LosingEvaluation));
}

#[test]
fn alpha_beta_chooses_repetition_draw_at_the_search_horizon() {
    assert_searcher_chooses_repetition_draw(&mut MiniMaxAlphaBetaSercher::new(LosingEvaluation));
}

#[test]
fn checkmate_takes_precedence_over_move_count_draw() {
    let mut board = Board::from_fen("7k/6Q1/5K2/8/8/8/8/8 b - - 100 1");
    assert_eq!(board.game_state(), GameState::Checkmate);
}
