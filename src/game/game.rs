use crate::board::{Board, MoveGenerator};
use crate::piece::{Color};
use crate::square::Square;
pub struct Game {
    pub board: Board,
    pub turn: Color,
}


impl Game {
    pub fn new() -> Self {
        Game{
            board: Board::default(),
            turn: Color::White
        }
    }

    pub fn make_move(&mut self, from: Square, to: Square) {
        let all_moves = MoveGenerator::generate_valid_moves(&mut self.board, self.turn);
        let next_move = all_moves
                                                                    .iter()
                                                                    .find(|mv| mv.from == from && mv.to == to);
        match next_move {
            Some(next_move) => {
                self.board.move_piece(*next_move);
                self.switch_turn();

            }
            None => {print!("Move from {from} to {to} was not a legal move"); return}
        }
        
    }
    pub fn show(&self) {
        self.board.show();
    }

    fn switch_turn(&mut self) {
        match self.turn {
            Color::White => self.turn = Color::Black,
            Color::Black => self.turn = Color::White
        }
    }


} // Impl Game
