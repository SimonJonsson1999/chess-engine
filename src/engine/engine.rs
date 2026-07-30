use crate::board::Board;
use crate::move_gen::MoveGenerator;
use crate::square::Square;
pub struct Engine {
    pub board: Board,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::default(),
        }
    }

    pub fn make_move(&mut self, from: Square, to: Square) {
        let all_moves = MoveGenerator::generate_valid_moves(&mut self.board);
        let next_move = all_moves.iter().find(|mv| mv.from == from && mv.to == to);
        match next_move {
            Some(next_move) => {
                self.board.move_piece(*next_move);
            }
            None => {
                print!("Move from {from} to {to} was not a legal move");
                return;
            }
        }
    }
    pub fn show(&self) {
        self.board.show();
    }
} // Impl Engine
