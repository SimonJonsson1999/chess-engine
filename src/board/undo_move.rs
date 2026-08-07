use crate::board::Board;
use crate::board::log::LogEntry;
use crate::board::piece::{Color, MoveKind, Piece, PieceType};
use crate::board::square::Square;
use crate::move_gen::BOARDWIDTH;
impl Board {
    pub fn undo(&mut self) {
        // If no moves in log, just return
        let Some(last_move) = self.move_log.remove() else {
            return;
        };
        debug_assert!(self.piece_on_square[last_move.to].is_some());
        self.switch_turn();
        // Set enpassant and castling rights to the previous values
        self.enpassant = last_move.previous_enpassant;
        self.castling_rights = last_move.previous_castling_rights;
        self.half_move = last_move.previous_half_move;
        self.full_move = last_move.previous_full_move;

        // get moved piece
        let moved_piece =
            self.piece_on_square[last_move.to].expect("Should always be a piece on the to square");
        match last_move.move_kind {
            MoveKind::KingCastle | MoveKind::QueenCastle => {
                self.undo_castle_move(last_move);
            }
            MoveKind::EnPassant => {
                self.undo_enpassant(last_move);
            }
            MoveKind::Capture => {
                // Restore captured piece, if any
                let captured_piece = last_move
                    .captured_piece
                    .expect("If Capture move there should be a captured piece");
                self.remove_piece(moved_piece, last_move.to);
                self.add_piece(captured_piece, last_move.to);
                self.add_piece(moved_piece, last_move.from);
            }
            MoveKind::Quiet | MoveKind::DoublePawnPush => {
                // Move the piece back
                self.remove_piece(moved_piece, last_move.to);
                self.add_piece(moved_piece, last_move.from);
            }
            MoveKind::PromotionCapture(_) => {
                let pawn = Piece::new(PieceType::Pawn, moved_piece.color);
                let captured_piece = last_move
                    .captured_piece
                    .expect("If Capture move there should be a captured piece");
                self.remove_piece(moved_piece, last_move.to);
                self.add_piece(captured_piece, last_move.to);
                self.add_piece(pawn, last_move.from);
            }
            MoveKind::Promotion(_) => {
                let pawn = Piece::new(PieceType::Pawn, moved_piece.color);
                self.remove_piece(moved_piece, last_move.to);
                self.add_piece(pawn, last_move.from);
            }
        }
    }

    fn undo_castle_move(&mut self, last_move: LogEntry) {
        let king = self.piece_on_square[last_move.to].expect("Expected king on destination");

        debug_assert_eq!(king.piece_type, PieceType::King);

        self.remove_piece(king, last_move.to);
        self.add_piece(king, last_move.from);

        let (rook_new_square, rook_old_square) = match (king.color, last_move.move_kind) {
            (Color::White, MoveKind::KingCastle) => (Square::H1, Square::F1),
            (Color::White, MoveKind::QueenCastle) => (Square::A1, Square::D1),
            (Color::Black, MoveKind::KingCastle) => (Square::H8, Square::F8),
            (Color::Black, MoveKind::QueenCastle) => (Square::A8, Square::D8),
            _ => unreachable!("Expected castle move"),
        };

        self.remove_piece(Piece::new(PieceType::Rook, king.color), rook_old_square);

        self.add_piece(Piece::new(PieceType::Rook, king.color), rook_new_square);
    }
    fn undo_enpassant(&mut self, last_move: LogEntry) {
        let pawn = self.piece_on_square[last_move.to].expect("Expected pawn on destination");

        debug_assert_eq!(pawn.piece_type, PieceType::Pawn);

        self.remove_piece(pawn, last_move.to);
        self.add_piece(pawn, last_move.from);

        let captured_square = match pawn.color {
            Color::White => Square::from_index(last_move.to.index() - BOARDWIDTH),
            Color::Black => Square::from_index(last_move.to.index() + BOARDWIDTH),
        };

        self.add_piece(
            Piece::new(PieceType::Pawn, pawn.color.opposite()),
            captured_square,
        );
    }
}
