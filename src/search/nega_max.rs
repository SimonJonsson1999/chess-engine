use crate::move_gen::MoveGenerator;
use crate::piece::{Color, PieceMove};
use crate::board::{Board};
use crate::evaluation::Evaluator;
use crate::search::Searcher;
use crate::board::GameState;

const CHECKMATE_SCORE: i32 = 30_000;
pub struct NegaMaxAlphaBetaSearcher<E: Evaluator> {
    evaluator: E,
}
impl<E: Evaluator> NegaMaxAlphaBetaSearcher<E> {
    pub fn new(evaluator: E) -> Self {
        Self { evaluator }
    }  
    pub fn negamax_alpha_beta(&self, board: &mut Board, depth: u8, mut alpha: i32, beta: i32) -> i32 {
        match board.game_state() {
            GameState::Checkmate => return -CHECKMATE_SCORE,
            GameState::Stalemate => return 0,
            _ => {}
        }
        if depth == 0 {
            return match board.turn {
                Color::White => self.evaluator.evaluate(board),
                Color::Black => -self.evaluator.evaluate(board)
            };
        }
        
        let moves = MoveGenerator::generate_valid_moves(board);
        let mut best = i32::MIN;

        for mv in moves {
            board.move_piece(mv);

            let score = -self.negamax_alpha_beta(board, depth - 1, -beta, -alpha);

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

impl<E: Evaluator> Searcher for NegaMaxAlphaBetaSearcher<E> {
    fn best_move(&mut self, board: &mut Board, depth: u8) -> PieceMove {
        debug_assert!(depth > 0);
        let mut best_move = None;
        let mut best_score = i32::MIN;

        let moves = MoveGenerator::generate_valid_moves(board);

        for mv in moves {
            board.move_piece(mv);

            let score = -self.negamax_alpha_beta(
                board,
                depth - 1,
                i32::MIN,
                i32::MAX,
            );

            board.undo();

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }

        best_move.expect("No legal moves")
    }
}