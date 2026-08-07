use crate::board::Board;
use crate::board::GameState;
use crate::board::PieceMoveList;
use crate::board::piece::{Color, PieceMove};
use crate::evaluation::Evaluator;
use crate::move_gen::MoveGenerator;
use crate::search::Searcher;
use crate::board::MoveKind;
use std::cmp::Reverse;
use rand::random;
const CHECKMATE_SCORE: i32 = 30_000;
const DRAW_SCORE: i32 = -50;
pub struct NegaMaxAlphaBetaSearcher<E: Evaluator> {
    evaluator: E,
}
impl<E: Evaluator> NegaMaxAlphaBetaSearcher<E> {
    pub fn new(evaluator: E) -> Self {
        Self { evaluator }
    }
    pub fn negamax_alpha_beta(
        &self,
        board: &mut Board,
        depth: u8,
        ply: u8,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        if depth == 0 {
            return match board.turn {
                Color::White => self.evaluator.evaluate(board),
                Color::Black => -self.evaluator.evaluate(board),
            };
        }

        let moves = self.ordered_moves(board);

        if moves.is_empty() {
            if board.is_in_check(board.turn) {
                return -CHECKMATE_SCORE + ply as i32;
            }
            return DRAW_SCORE;
        }

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

            alpha = alpha.max(score);

            if alpha >= beta {
                break;
            }
        }

        alpha
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

            let score = -self.negamax_alpha_beta(board, depth - 1,0, i32::MIN, i32::MAX);

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

