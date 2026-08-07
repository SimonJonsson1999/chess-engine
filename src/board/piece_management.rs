use crate::board::Board;
use crate::board::piece::{Color, Piece};
use crate::board::square::Square;
impl Board {
    pub(crate) fn add_piece(&mut self, piece: Piece, sq: Square) {
        self.bitboards[piece].set(sq);

        match piece.color {
            Color::White => self.white.set(sq),
            Color::Black => self.black.set(sq),
        }

        self.empty.clear(sq);
        self.piece_on_square[sq] = Some(piece);
    }

    pub(crate) fn remove_piece(&mut self, piece: Piece, sq: Square) {
        self.bitboards[piece].clear(sq);

        match piece.color {
            Color::White => self.white.clear(sq),
            Color::Black => self.black.clear(sq),
        }

        self.empty.set(sq);
        self.piece_on_square[sq] = None;
    }
}
