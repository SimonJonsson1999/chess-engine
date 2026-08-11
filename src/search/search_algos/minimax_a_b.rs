use crate::board::Board;
use crate::board::GameState;
use crate::board::piece::{Color, PieceMove};
use crate::evaluation::Evaluator;
use crate::move_gen::MoveGenerator;
use crate::search::Searcher;

const CHECKMATE_SCORE: i32 = 30_000;
pub struct MiniMaxAlphaBetaSercher<E: Evaluator> {
    evaluator: E,
}
impl<E: Evaluator> MiniMaxAlphaBetaSercher<E> {
    pub fn new(evaluator: E) -> Self {
        Self { evaluator }
    }
    pub fn minimax_alpha_beta(
        &self,
        board: &mut Board,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        if depth == 0 {
            return self.evaluator.evaluate(board);
        }

        match board.game_state() {
            GameState::Checkmate => match board.turn {
                Color::White => return -CHECKMATE_SCORE,
                Color::Black => return CHECKMATE_SCORE,
            },
            GameState::Stalemate => return 0,
            _ => {}
        }
        let moves = MoveGenerator::generate_valid_moves(board);
        match board.turn {
            Color::White => {
                let mut best = i32::MIN;

                for mv in moves {
                    board.move_piece(mv);

                    let score = self.minimax_alpha_beta(board, depth - 1, alpha, beta);

                    board.undo();

                    if score >= beta {
                        return beta;
                    }
                    alpha = alpha.max(score);
                    best = best.max(score);
                }

                best
            }

            Color::Black => {
                let mut best = i32::MAX;

                for mv in moves {
                    board.move_piece(mv);

                    let score = self.minimax_alpha_beta(board, depth - 1, alpha, beta);

                    board.undo();
                    if score <= alpha {
                        return alpha;
                    }
                    beta = beta.min(score);
                    best = best.min(score);
                }

                best
            }
        }
    }
}

impl<E: Evaluator> Searcher for MiniMaxAlphaBetaSercher<E> {
    fn best_move(&mut self, board: &mut Board, depth: u8) -> PieceMove {
        debug_assert!(depth > 0);
        let mut best_move = None;
        let mut best_score = i32::MIN;

        let moves = MoveGenerator::generate_valid_moves(board);

        for mv in moves {
            board.move_piece(mv);

            let score = self.minimax_alpha_beta(board, depth - 1, i32::MIN, i32::MAX);

            board.undo();

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }

        best_move.expect("No legal moves")
    }
}

// https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
