use crate::board::Board;
use crate::log::LogEntry;
use crate::move_gen::BOARDWIDTH;
use crate::piece::{Color, MoveKind, Piece, PieceMove, PieceType};
use crate::square::Square;
impl Board {
    pub fn move_piece(&mut self, piece_move: PieceMove) {
        // extract squares
        let from_sq = piece_move.from;
        let to_sq = piece_move.to;

        // Save previous castling and enpassant, and move counters
        let previous_castling_rights = self.castling_rights;
        let previous_enpassant = self.enpassant;
        let previous_half_move = self.half_move;
        let previous_full_move = self.full_move;

        // Extract moved and captured pieces
        let from_piece = self.piece_on_square[from_sq].expect("No piece on from square");
        let mut captured_piece: Option<Piece> = self.piece_on_square[to_sq];

        self.update_enpassant_square(piece_move);

        // Move pieces depending on kind of move
        match piece_move.kind {
            MoveKind::Promotion(promotion_piece_type) => {
                debug_assert!(captured_piece.is_none());
                self.remove_piece(from_piece, from_sq);
                let promoted_piece = Piece::new(promotion_piece_type, from_piece.color);
                self.add_piece(promoted_piece, to_sq);
            }
            MoveKind::PromotionCapture(promotion_piece_type) => {
                let captured =
                    captured_piece.expect("Expected captured piece for promotion capture");
                self.remove_piece(captured, to_sq);
                self.remove_piece(from_piece, from_sq);
                let promoted_piece = Piece::new(promotion_piece_type, from_piece.color);
                self.add_piece(promoted_piece, to_sq);
            }
            MoveKind::EnPassant => {
                self.make_enpassant_move(piece_move);
                captured_piece = Some(Piece::new(PieceType::Pawn, from_piece.color.opposite()));
            }
            MoveKind::KingCastle | MoveKind::QueenCastle => {
                self.make_castle_move(piece_move);
                captured_piece = None;
            }
            MoveKind::Quiet | MoveKind::DoublePawnPush => {
                debug_assert!(captured_piece.is_none());
                self.remove_piece(from_piece, from_sq);
                self.add_piece(from_piece, to_sq);
            }
            MoveKind::Capture => {
                let captured = captured_piece.expect("Expected captured piece for capture");
                self.remove_piece(captured, to_sq);
                self.remove_piece(from_piece, from_sq);
                self.add_piece(from_piece, to_sq);
            }
        };

        if from_piece.piece_type == PieceType::Pawn || captured_piece.is_some() {
            self.half_move = 0;
        } else {
            self.half_move += 1;
        }
        // update castling rights
        self.update_castling_rights(from_sq, to_sq, from_piece, captured_piece);
        // Add a log entry to the move log
        self.move_log.add(LogEntry::new(
            from_sq,
            to_sq,
            captured_piece,
            previous_enpassant,
            previous_castling_rights,
            piece_move.kind,
            previous_half_move,
            previous_full_move,
        ));
        // udpate full move counter if it was blacks turn
        match self.turn {
            Color::Black => self.full_move += 1,
            _ => {}
        }

        // Switch the turn after a succesful move
        self.switch_turn()
    }

    fn make_castle_move(&mut self, piece_move: PieceMove) {
        match (piece_move.kind, piece_move.from) {
            (MoveKind::KingCastle, Square::E1) => {
                // White king: E1 -> G1
                self.remove_piece(Piece::new(PieceType::King, Color::White), Square::E1);
                self.add_piece(Piece::new(PieceType::King, Color::White), Square::G1);

                // White rook: H1 -> F1
                self.remove_piece(Piece::new(PieceType::Rook, Color::White), Square::H1);
                self.add_piece(Piece::new(PieceType::Rook, Color::White), Square::F1);
            }

            (MoveKind::QueenCastle, Square::E1) => {
                // White king: E1 -> C1
                self.remove_piece(Piece::new(PieceType::King, Color::White), Square::E1);
                self.add_piece(Piece::new(PieceType::King, Color::White), Square::C1);

                // White rook: A1 -> D1
                self.remove_piece(Piece::new(PieceType::Rook, Color::White), Square::A1);
                self.add_piece(Piece::new(PieceType::Rook, Color::White), Square::D1);
            }

            (MoveKind::KingCastle, Square::E8) => {
                // Black king: E8 -> G8
                self.remove_piece(Piece::new(PieceType::King, Color::Black), Square::E8);
                self.add_piece(Piece::new(PieceType::King, Color::Black), Square::G8);

                // Black rook: H8 -> F8
                self.remove_piece(Piece::new(PieceType::Rook, Color::Black), Square::H8);
                self.add_piece(Piece::new(PieceType::Rook, Color::Black), Square::F8);
            }

            (MoveKind::QueenCastle, Square::E8) => {
                // Black king: E8 -> C8
                self.remove_piece(Piece::new(PieceType::King, Color::Black), Square::E8);
                self.add_piece(Piece::new(PieceType::King, Color::Black), Square::C8);

                // Black rook: A8 -> D8
                self.remove_piece(Piece::new(PieceType::Rook, Color::Black), Square::A8);
                self.add_piece(Piece::new(PieceType::Rook, Color::Black), Square::D8);
            }

            _ => unreachable!("Invalid castle move"),
        }
    }

    fn make_enpassant_move(&mut self, piece_move: PieceMove) {
        let pawn = self.piece_on_square[piece_move.from].expect("Expected pawn");
        let capture_square = match pawn.color {
            Color::White => {
                self.remove_piece(pawn, piece_move.from);
                self.add_piece(pawn, piece_move.to);
                Square::from_index(piece_move.to.index() - BOARDWIDTH)
            }
            Color::Black => {
                self.remove_piece(pawn, piece_move.from);
                self.add_piece(pawn, piece_move.to);
                Square::from_index(piece_move.to.index() + BOARDWIDTH)
            }
        };
        let captured_pawn = self.piece_on_square[capture_square].expect("Expected captured pawn");
        debug_assert_eq!(captured_pawn.piece_type, PieceType::Pawn);
        debug_assert_eq!(captured_pawn.color, pawn.color.opposite());
        self.remove_piece(captured_pawn, capture_square);
    }

    fn update_enpassant_square(&mut self, piece_move: PieceMove) {
        self.enpassant = None;
        let Some(piece) = self.piece_on_square[piece_move.from] else {
            return;
        };
        if piece.piece_type != PieceType::Pawn {
            return;
        }
        if piece_move.kind == MoveKind::DoublePawnPush {
            let passed_square_index = (piece_move.from.index() + piece_move.to.index()) / 2;
            self.enpassant = Some(Square::from_index(passed_square_index));
        }
    }

    fn update_castling_rights(
        &mut self,
        from: Square,
        to: Square,
        from_piece: Piece,
        captured_piece: Option<Piece>,
    ) {
        if from_piece.piece_type == PieceType::King {
            match from_piece.color {
                Color::White => {
                    self.castling_rights.white_kingside = false;
                    self.castling_rights.white_queenside = false;
                }
                Color::Black => {
                    self.castling_rights.black_kingside = false;
                    self.castling_rights.black_queenside = false;
                }
            }
        }
        // Rook moved
        if from_piece.piece_type == PieceType::Rook {
            match from {
                Square::A1 => self.castling_rights.white_queenside = false,
                Square::H1 => self.castling_rights.white_kingside = false,
                Square::A8 => self.castling_rights.black_queenside = false,
                Square::H8 => self.castling_rights.black_kingside = false,
                _ => {}
            }
        }
        // Rook captured
        if let Some(captured) = captured_piece {
            if captured.piece_type == PieceType::Rook {
                match to {
                    Square::A1 => self.castling_rights.white_queenside = false,
                    Square::H1 => self.castling_rights.white_kingside = false,
                    Square::A8 => self.castling_rights.black_queenside = false,
                    Square::H8 => self.castling_rights.black_kingside = false,
                    _ => {}
                }
            }
        }
    }
}
