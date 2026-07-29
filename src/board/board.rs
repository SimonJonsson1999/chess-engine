use crate::bb;
use crate::bitboard::{BitBoard, BitBoards};
use crate::board::MoveGenerator;
use crate::log::{Move, MoveLog};
use crate::piece::{Color, MoveKind, Piece, PieceMove, PieceType};
use crate::square::{Square, SquareMap};

// All pieces are represented using 6*2 bitboards, for the 6 different pieces
// for both colors
// Each bitboard is 64 bits, where each bit will represent if the piece is present or not
pub struct Board {
    pub bitboards: BitBoards,
    piece_on_square: SquareMap<Option<Piece>>,
    pub empty: BitBoard,
    pub white: BitBoard,
    pub black: BitBoard,
    move_log: MoveLog,
    pub enpassant: Option<Square>,
}

impl Board {
    pub fn new() -> Self {
        // bitboards hold 12 bitboards representing the pieces
        // the bitborads can be indexed using either a color and a piecetype, such as below
        // or straigt up using a piece, which holds both type and color info
        let mut bitboards = BitBoards::default();
        let move_log = MoveLog::new();
        let enpassant = None;

        // Create 12 BitBoards, one for each combination of color and piece type
        // White
        bitboards[Color::White][PieceType::Pawn] = bb!(A2, B2, C2, D2, E2, F2, G2, H2);
        bitboards[Color::White][PieceType::Knight] = bb!(B1, G1);
        bitboards[Color::White][PieceType::Bishop] = bb!(C1, F1);
        bitboards[Color::White][PieceType::Rook] = bb!(A1, H1);
        bitboards[Color::White][PieceType::Queen] = bb!(D1);
        bitboards[Color::White][PieceType::King] = bb!(E1);

        // Black
        bitboards[Color::Black][PieceType::Pawn] = bb!(A7, B7, C7, D7, E7, F7, G7, H7);
        bitboards[Color::Black][PieceType::Knight] = bb!(B8, G8);
        bitboards[Color::Black][PieceType::Bishop] = bb!(C8, F8);
        bitboards[Color::Black][PieceType::Rook] = bb!(A8, H8);
        bitboards[Color::Black][PieceType::Queen] = bb!(D8);
        bitboards[Color::Black][PieceType::King] = bb!(E8);

        let mut piece_on_square = SquareMap::filled(None);
        // init empty as all squares empty, set the taken squares in the following loop
        // Compute occupancy bitboards
        let white = bitboards[Color::White][PieceType::Pawn]
            | bitboards[Color::White][PieceType::Knight]
            | bitboards[Color::White][PieceType::Bishop]
            | bitboards[Color::White][PieceType::Rook]
            | bitboards[Color::White][PieceType::Queen]
            | bitboards[Color::White][PieceType::King];

        let black = bitboards[Color::Black][PieceType::Pawn]
            | bitboards[Color::Black][PieceType::Knight]
            | bitboards[Color::Black][PieceType::Bishop]
            | bitboards[Color::Black][PieceType::Rook]
            | bitboards[Color::Black][PieceType::Queen]
            | bitboards[Color::Black][PieceType::King];

        let empty = !(white | black);

        for color in [Color::White, Color::Black] {
            for piece_type in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ] {
                let bb = bitboards[color][piece_type];

                for square in 0..64 {
                    if (bb.0 & (1u64 << square)) != 0 {
                        piece_on_square[Square::from_index(square)] =
                            Some(Piece::new(piece_type, color));
                    }
                }
            }
        }
        Self {
            bitboards,
            piece_on_square,
            empty,
            white,
            black,
            move_log,
            enpassant,
        }
    }

    pub fn move_piece(&mut self, piece_move: PieceMove) {
        // Get what piece should be moved, return if None
        let from_sq = piece_move.from;
        let to_sq = piece_move.to;
        // is this needed??
        // I think we have a bug if this is not always true
        let from_piece = match self.piece_on_square[from_sq] {
            Some(piece) => piece,
            None => {
                return;
            }
        };
        let previous_enpassant = self.enpassant;
        self.update_enpassant_square(piece_move);
        // Get captured piece and clear position in bitboard if piece found of different color
        // Capture
        let captured_piece = self.piece_on_square[to_sq];

        if let Some(piece) = captured_piece {
            if piece.color == from_piece.color {
                return;
            }

            self.remove_piece(piece, to_sq);
        }

        // Move piece
        match piece_move.kind {
            MoveKind::Promotion(piece_type) | MoveKind::PromotionCapture(piece_type) => {
                self.remove_piece(from_piece, from_sq);

                let promoted_piece = Piece::new(piece_type, from_piece.color);
                self.add_piece(promoted_piece, to_sq);
            }

            _ => {
                self.remove_piece(from_piece, from_sq);
                self.add_piece(from_piece, to_sq);
            }
        }
        // Add entry to log
        self.move_log.add(Move::new(
            from_sq,
            to_sq,
            captured_piece,
            previous_enpassant,
        ));
    }

    pub fn undo(&mut self) {
        let Some(last_move) = self.move_log.remove() else {
            return;
        };

        let Some(moved_piece) = self.piece_on_square[last_move.to] else {
            return;
        };
        self.enpassant = last_move.previous_enpassant;

        // Move the piece back
        self.remove_piece(moved_piece, last_move.to);
        self.add_piece(moved_piece, last_move.from);

        // Restore captured piece, if any
        if let Some(captured_piece) = last_move.captured_piece {
            self.add_piece(captured_piece, last_move.to);
        }
    }

    fn add_piece(&mut self, piece: Piece, sq: Square) {
        self.bitboards[piece].set(sq);

        match piece.color {
            Color::White => self.white.set(sq),
            Color::Black => self.black.set(sq),
        }

        self.empty.clear(sq);
        self.piece_on_square[sq] = Some(piece);
    }

    fn remove_piece(&mut self, piece: Piece, sq: Square) {
        self.bitboards[piece].clear(sq);

        match piece.color {
            Color::White => self.white.clear(sq),
            Color::Black => self.black.clear(sq),
        }

        self.empty.set(sq);
        self.piece_on_square[sq] = None;
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

    pub fn show(&self) {
        let mut piece_names: [Option<(Color, PieceType)>; 64] = [None; 64];
        for color in [Color::White, Color::Black] {
            for piece in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ] {
                let bb = &self.bitboards[color][piece];
                for square in 0..64 {
                    if (bb.0 & (1u64 << square)) != 0 {
                        piece_names[square] = Some((color, piece))
                    }
                }
            }
        }
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);

            for file in 0..8 {
                let square = rank * 8 + file;

                match piece_names[square] {
                    Some((color, piece)) => print!("{}{} ", color, piece),
                    None => print!(".. "),
                }
            }

            println!(); // <- this must execute once per rank
        }

        println!("  A  B  C  D  E  F  G  H");
    }

    pub fn print_log(&self) {
        self.move_log.print_entries();
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
