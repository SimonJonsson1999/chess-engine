use crate::board::Board;
use crate::board::piece::PieceMove;
use crate::search::Searcher;

pub trait AI {
    fn choose_move(&mut self, board: &mut Board) -> PieceMove;
}

pub struct Computer<S: Searcher> {
    searcher: S,
    depth: u8,
}

impl<S: Searcher> Computer<S> {
    pub fn new(searcher: S, depth: u8) -> Self {
        Self { searcher, depth }
    }
}

impl<S: Searcher> AI for Computer<S> {
    fn choose_move(&mut self, board: &mut Board) -> PieceMove {
        self.searcher.best_move(board, self.depth)
    }
}
