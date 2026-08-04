use crate::evaluation::BoardEvaluation;
use crate::move_gen::MoveGenerator;
use crate::piece::{Color, PieceMove};
use crate::board::Board;
pub struct Search;

impl Search {
    pub fn best_move(board: &mut Board) -> PieceMove {
        let turn = board.turn;
        let mut best_move = None;
        let mut best_score = match turn {
            Color::White => i32::MIN,
            Color::Black => i32::MAX,
        };

        let moves = MoveGenerator::generate_valid_moves(board);

        for mv in moves {
            board.move_piece(mv);

            let score = BoardEvaluation::evaluate(board);

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


}