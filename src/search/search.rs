use crate::evaluation::BoardEvaluation;
use crate::move_gen::MoveGenerator;
use crate::piece::{Color, PieceMove};
use crate::board::{Board, GameState};
pub struct Search;
const CHECKMATE_SCORE: i32 = 30_000;
impl Search {
    pub fn best_move(board: &mut Board, depth: u8) -> PieceMove {
        debug_assert!(depth > 0);
        let turn = board.turn;
        let mut best_move = None;
        let mut best_score = match turn {
            Color::White => i32::MIN,
            Color::Black => i32::MAX,
        };

        let moves = MoveGenerator::generate_valid_moves(board);

        for mv in moves {
            board.move_piece(mv);

            // let score = Self::minimax(board, depth-1);
            let score = Self::minimax_alpha_beta(board, depth-1, i32::MIN, i32::MAX);
            board.undo();

            let better = match turn {
                Color::White => score > best_score,
                Color::Black => score < best_score,
            };

            if better {
                best_score = score;
                best_move = Some(mv);
            }
        }
        best_move.expect("No legal moves")
    }

    // Algortihm found here https://en.wikipedia.org/wiki/Minimax
    fn minimax(board: &mut Board, depth: u8) -> i32 {
        if depth == 0 {
            return BoardEvaluation::evaluate(board);
        }
        match board.game_state() {
            GameState::Checkmate => {
                match board.turn {
                    Color::White => {
                        return -CHECKMATE_SCORE
                    },
                    Color::Black => {
                        return CHECKMATE_SCORE
                    }
                }
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

                    let score = Self::minimax(board, depth - 1);

                    board.undo();

                    best = best.max(score);
                }

                best
            }

            Color::Black => {
                let mut best = i32::MAX;

                for mv in moves {
                    board.move_piece(mv);

                    let score = Self::minimax(board, depth - 1);

                    board.undo();

                    best = best.min(score);
                }

                best
            }
        }
    }
    // https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning
    pub fn minimax_alpha_beta(board: &mut Board, depth: u8, mut alpha: i32, mut beta: i32) -> i32 {
        if depth == 0 {
            return BoardEvaluation::evaluate(board);
        }

        

        match board.game_state() {
            GameState::Checkmate => {
                match board.turn {
                    Color::White => {
                        return -CHECKMATE_SCORE
                    },
                    Color::Black => {
                        return CHECKMATE_SCORE
                    }
                }
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

                    let score = Self::minimax_alpha_beta(board, depth - 1, alpha, beta);

                    board.undo();
                    
                    if score >= beta {
                        return beta
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

                    let score = Self::minimax_alpha_beta(board, depth - 1, alpha, beta);

                    board.undo();
                    if score <= alpha {
                        return alpha
                    }
                    beta = beta.min(score);
                    best = best.min(score);
                }

                best
            }
        }
    }

    pub fn negamax_alpha_beta(board: &mut Board, depth: u8, mut alpha: i32, mut beta: i32) -> i32 {
        match board.game_state() {
            GameState::Checkmate => return -CHECKMATE_SCORE,
            GameState::Stalemate => return 0,
            _ => {}
        }
        if depth == 0 {
            return match board.turn {
                Color::White => BoardEvaluation::evaluate(board),
                Color::Black => -BoardEvaluation::evaluate(board),
            };
        }
        
        let moves = MoveGenerator::generate_valid_moves(board);
        let mut best = i32::MIN;

        for mv in moves {
            board.move_piece(mv);

            let score = -Self::negamax_alpha_beta(board, depth - 1, -beta, -alpha);

            board.undo();
            best = best.max(score);
            alpha = alpha.max(score);

            if alpha >= beta {
                break;
            }
            
        }

        best
    }       
}
