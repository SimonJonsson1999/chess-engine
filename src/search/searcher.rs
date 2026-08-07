use crate::board::Board;
use crate::board::piece::PieceMove;

pub trait Searcher {
    fn best_move(&mut self, board: &mut Board, depth: u8) -> PieceMove;
}
