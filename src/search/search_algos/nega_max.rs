use crate::board::Board;
use crate::board::piece::{Color, PieceMove};
use crate::evaluation::Evaluator;
use crate::move_gen::MoveGenerator;
use crate::search::Searcher;
use crate::board::MoveKind;
use crate::search::{TranspositionTable, TTEntry, Bound};
use std::cmp::Reverse;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use rand::random;
const CHECKMATE_SCORE: i32 = 30_000;
const DRAW_SCORE: i32 = 0;
pub struct NegaMaxAlphaBetaSearcher<E: Evaluator> {
    evaluator: E,
    tt: TranspositionTable,
}
impl<E: Evaluator> NegaMaxAlphaBetaSearcher<E> {
    pub fn new(evaluator: E) -> Self {
        let tt = TranspositionTable::new(64);
        Self { evaluator, tt }
    }
    pub fn negamax_alpha_beta(
        &mut self,
        board: &mut Board,
        depth: u8,
        ply: u8,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        
        let moves = self.ordered_moves(board);

        if moves.is_empty() {
            if board.is_in_check(board.turn) {
                return -CHECKMATE_SCORE + ply as i32;
            }
            return DRAW_SCORE;
        }
        // Draws depend on the played history, including at the search horizon.
        if board.is_threefold_repetition() || board.is_fifty_move_rule() {
            return DRAW_SCORE;
        }
        if depth == 0 {
            return match board.turn {
                Color::White => self.evaluator.evaluate(board),
                Color::Black => -self.evaluator.evaluate(board),
            };
        }

        let hash = Self::transposition_key(board);
        let alpha_original = alpha;
        let beta_original = beta;
        if let Some(entry) = self.tt.get(hash) {
            if entry.depth >= depth {
                match entry.bound {
                    Bound::Exact => return entry.score,
                    Bound::Lower => alpha = alpha.max(entry.score),
                    Bound::Upper => beta = beta.min(entry.score),
                }

                if alpha >= beta {
                    return entry.score;
                }
            }
        }

        let mut best_score = i32::MIN;
        for mv in moves {
            board.move_piece(mv);
            let score = -self.negamax_alpha_beta(
                board,
                depth - 1,
                ply + 1,
                -beta,
                -alpha,
            );

            board.undo();

            best_score = best_score.max(score);
            alpha = alpha.max(score);

            if alpha >= beta {
                break;
            }
        }
        let bound = if best_score <= alpha_original {
            Bound::Upper
        } else if best_score >= beta_original {
            Bound::Lower
        } else {
            Bound::Exact
        };

        self.tt.store(TTEntry {
            hash,
            depth,
            score: best_score,
            bound,
        });

        best_score
    }

    fn transposition_key(board: &Board) -> u64 {
        // The same position can have a different score with a different draw
        // history. Include reversible history and the fifty-move counter.
        let mut key = DefaultHasher::new();
        key.write_u64(board.zobrist_hash);
        key.write_u8(board.half_move);
        for entry in board.move_log.iter().rev().take(board.half_move as usize) {
            key.write_u64(entry.previous_zobrist_hash);
        }
        key.finish()
    }

    fn ordered_moves(&self, board: &mut Board) -> Vec<PieceMove> {
        let mut moves: Vec<ScoredMove> = MoveGenerator::generate_valid_moves(board)
            .into_iter()
            .map(|mv| ScoredMove {
                score: Self::score_move(board, mv),
                mv,
            })
            .collect();

        moves.sort_unstable_by_key(|m| Reverse(m.score));

        moves.into_iter().map(|m| m.mv).collect()
    }

    fn score_move(board: &Board, mv: PieceMove) -> i32 {
        match mv.kind {
            MoveKind::Promotion(_) => 10_000,

            MoveKind::Capture => {
                let victim = board.piece_on_square[mv.to]
                    .expect("Capture without victim");

                let attacker = board.piece_on_square[mv.from]
                    .expect("No attacking piece");

                // MVV-LVA
                10 * victim.piece_type.value() - attacker.piece_type.value()
            }

            _ => 0,
        }
    }
}

struct ScoredMove {
    mv: PieceMove,
    score: i32,
}


impl<E: Evaluator> Searcher for NegaMaxAlphaBetaSearcher<E> {
    fn best_move(&mut self, board: &mut Board, depth: u8) -> PieceMove {
        debug_assert!(depth > 0);
        let mut best_move = None;
        let mut best_score = i32::MIN;

        let mut moves: Vec<ScoredMove> = MoveGenerator::generate_valid_moves(board)
                                                                                .into_iter()
                                                                                .map(|mv| ScoredMove {
                                                                                    score: Self::score_move(board, mv),
                                                                                    mv,
                                                                                })
                                                                                .collect();

        moves.sort_unstable_by_key(|m| Reverse(m.score));
        // score moves 



        // sort_moves

        for scored_move in moves {
            let mv = scored_move.mv;
            board.move_piece(mv);

            // Both bounds must be representable when negated in recursive calls.
            let score = -self.negamax_alpha_beta(board, depth - 1,1, -i32::MAX, i32::MAX);

            board.undo();

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            } else if score == best_score && random::<bool>() {
                best_move = Some(mv);
                best_score = score;
            }
            // println!("{:?} -> {}", mv, score);
        }
        // println!("{:?} -> {}", best_move, best_score);
        best_move.expect("No legal moves")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Square;
    use crate::evaluation::BoardEvaluation;

    struct ConstantEvaluation;

    impl Evaluator for ConstantEvaluation {
        fn evaluate(&self, _board: &Board) -> i32 {
            123
        }
    }

    fn play_knight_cycle(board: &mut Board, plies: usize) {
        let cycle = [
            (Square::G1, Square::F3),
            (Square::G8, Square::F6),
            (Square::F3, Square::G1),
            (Square::F6, Square::G8),
        ];
        for &(from, to) in cycle.iter().cycle().take(plies) {
            board.move_piece(PieceMove::new(from, to, MoveKind::Quiet));
        }
    }

    #[test]
    fn the_same_quiet_move_in_different_positions_does_not_reuse_scores() {
        let without_queen = "4k3/8/8/8/8/8/8/4K1N1 w - - 0 1";
        let with_queen = "4k3/8/8/8/8/8/8/3QK1N1 w - - 0 1";
        let mut shared = NegaMaxAlphaBetaSearcher::new(BoardEvaluation);

        for fen in [without_queen, with_queen, without_queen] {
            let mut board = Board::from_fen(fen);
            board.move_piece(PieceMove::new(Square::G1, Square::F3, MoveKind::Quiet));
            let mut reference = Board::from_fen(fen);
            reference.move_piece(PieceMove::new(Square::G1, Square::F3, MoveKind::Quiet));
            let mut fresh = NegaMaxAlphaBetaSearcher::new(BoardEvaluation);
            let expected = fresh.negamax_alpha_beta(&mut reference, 1, 0, -i32::MAX, i32::MAX);
            let actual = shared.negamax_alpha_beta(&mut board, 1, 0, -i32::MAX, i32::MAX);

            assert_eq!(actual, expected, "Cached score leaked into {fen}");
            assert_eq!(board, reference, "Search must restore the board and history");
        }
    }

    #[test]
    fn repetition_is_a_draw_at_all_depths_even_with_a_cached_score() {
        let mut board = Board::default();
        play_knight_cycle(&mut board, 8);
        let mut searcher = NegaMaxAlphaBetaSearcher::new(ConstantEvaluation);
        searcher.tt.store(TTEntry {
            hash: NegaMaxAlphaBetaSearcher::<ConstantEvaluation>::transposition_key(&board),
            depth: 4,
            score: 999,
            bound: Bound::Exact,
        });

        for depth in [0, 1, 3] {
            assert_eq!(
                searcher.negamax_alpha_beta(&mut board, depth, 0, -i32::MAX, i32::MAX),
                DRAW_SCORE,
            );
        }
    }

    #[test]
    fn transposition_scores_do_not_leak_between_repetition_histories() {
        let mut repeated = Board::default();
        play_knight_cycle(&mut repeated, 7);
        let mut fresh = Board::from_fen(
            "rnbqkb1r/pppppppp/5n2/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 7 4",
        );
        assert_eq!(repeated.zobrist_hash, fresh.zobrist_hash);
        assert_eq!(repeated.half_move, fresh.half_move);
        let mut searcher = NegaMaxAlphaBetaSearcher::new(ConstantEvaluation);

        assert_eq!(
            searcher.negamax_alpha_beta(&mut fresh, 1, 0, -i32::MAX, i32::MAX),
            -123,
        );
        assert_eq!(
            searcher.negamax_alpha_beta(&mut repeated, 1, 0, -i32::MAX, i32::MAX),
            DRAW_SCORE,
        );
        assert_eq!(
            searcher.negamax_alpha_beta(&mut fresh, 1, 0, -i32::MAX, i32::MAX),
            -123,
        );
    }

    #[test]
    fn transposition_scores_respect_the_fifty_move_counter() {
        let mut fresh = Board::from_fen("4k1n1/8/8/8/8/8/8/4K1N1 w - - 0 1");
        let mut near_draw = Board::from_fen("4k1n1/8/8/8/8/8/8/4K1N1 w - - 99 1");
        assert_eq!(fresh.zobrist_hash, near_draw.zobrist_hash);
        let mut searcher = NegaMaxAlphaBetaSearcher::new(ConstantEvaluation);

        assert_eq!(
            searcher.negamax_alpha_beta(&mut fresh, 1, 0, -i32::MAX, i32::MAX),
            123,
        );
        assert_eq!(
            searcher.negamax_alpha_beta(&mut near_draw, 1, 0, -i32::MAX, i32::MAX),
            DRAW_SCORE,
        );
    }

    #[test]
    fn best_move_at_match_depth_does_not_overflow() {
        let mut board = Board::default();
        let original_hash = board.zobrist_hash;
        let legal_moves = MoveGenerator::generate_valid_moves(&mut board);
        let mut searcher = NegaMaxAlphaBetaSearcher::new(BoardEvaluation {});

        let mv = searcher.best_move(&mut board, 3);

        assert!(legal_moves.iter().any(|legal_move| *legal_move == mv));
        assert_eq!(board.zobrist_hash, original_hash);
        assert_eq!(board.turn, Color::White);
    }
}

