use crate::bitboard::BitBoard;
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

    pub fn make_move(&mut self, from: Square, to: Square) -> bool{
        let all_moves = MoveGenerator::generate_valid_moves(&mut self.board);
        let next_move = all_moves.iter().find(|mv| mv.from == from && mv.to == to);
        match next_move {
            Some(next_move) => {
                self.board.move_piece(*next_move);
                return true
            }
            None => {
                print!("Move from {from} to {to} was not a legal move");
                return false
            }
        }
    }
    pub fn show(&self) {
        self.board.show();
    }

    pub fn attacked_squares(&mut self, square: Square) -> Vec<Square>{
        let attacked: BitBoard = MoveGenerator::generate_valid_moves_from(&mut self.board, square)
                                                                    .into_iter()
                                                                    .fold(BitBoard(0), |mut bb, mv| {
                                                                        bb.set(mv.to);
                                                                        bb
                                                                    });
        let moves_from_sq = attacked.squares();
        moves_from_sq
    }
} // Impl Engine
