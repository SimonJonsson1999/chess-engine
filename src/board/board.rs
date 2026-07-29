use crate::bb;
use crate::board::BOARDWIDTH;
use crate::bitboard::{BitBoard, BitBoards};
use crate::log::{LogEntry, MoveLog};
use crate::piece::{Color, MoveKind, Piece, PieceMove, PieceType};
use crate::square::{Square, SquareMap};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}
impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights{
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true
        }
    }
}
// All pieces are represented using 6*2 bitboards, for the 6 different pieces
// for both colors
// Each bitboard is 64 bits, where each bit will represent if the piece is present or not
pub struct Board {
    pub bitboards: BitBoards,
    pub piece_on_square: SquareMap<Option<Piece>>,
    pub empty: BitBoard,
    pub white: BitBoard,
    pub black: BitBoard,
    move_log: MoveLog,
    pub enpassant: Option<Square>,
    pub castling_rights: CastlingRights,
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
            castling_rights: CastlingRights::new(),
        }
    }

    pub fn move_piece(&mut self, piece_move: PieceMove) {
        // Get what piece should be moved, return if None
        let from_sq = piece_move.from;
        let to_sq = piece_move.to;
        let from_piece = self.piece_on_square[from_sq].expect("No piece on from square");
        let previous_castling_rights = self.castling_rights;
        let previous_enpassant = self.enpassant;
        if piece_move.kind == MoveKind::KingCastle || piece_move.kind == MoveKind::QueenCastle {
            self.make_castle_move(piece_move);
            self.update_castling_rights(from_sq, to_sq, from_piece, None);
            
            // Add entry to log
            self.move_log.add(LogEntry::new(
                from_sq,
                to_sq,
                None,
                previous_enpassant,
                previous_castling_rights,
                piece_move.kind

            ));
            return
        }
        

        self.update_enpassant_square(piece_move);
        // Get captured piece and clear position in bitboard if piece found of different color
        // Capture
        if piece_move.kind == MoveKind::EnPassant {
            self.make_enpassant_move(piece_move);
            let captured_piece = Some(Piece::new(PieceType::Pawn, from_piece.color.opposite()));
            self.update_castling_rights(from_sq, to_sq, from_piece, captured_piece);
            self.move_log.add(LogEntry::new(
                from_sq,
                to_sq,
                captured_piece,
                previous_enpassant,
                previous_castling_rights,
                piece_move.kind

            ));
            return
        }
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
        self.update_castling_rights(from_sq, to_sq, from_piece, captured_piece);
        // Add entry to log
        self.move_log.add(LogEntry::new(
            from_sq,
            to_sq,
            captured_piece,
            previous_enpassant,
            previous_castling_rights,
            piece_move.kind

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
        self.castling_rights = last_move.previous_castling_rights;

        if last_move.move_kind == MoveKind::KingCastle || last_move.move_kind == MoveKind::QueenCastle {
            self.undo_castle_move(last_move);
            return
        }
        if last_move.move_kind == MoveKind::EnPassant {
            self.undo_enpassant(last_move);
            return
        }
        // Move the piece back
        self.remove_piece(moved_piece, last_move.to);
        self.add_piece(moved_piece, last_move.from);

        // Restore captured piece, if any
        if let Some(captured_piece) = last_move.captured_piece {
            self.add_piece(captured_piece, last_move.to);
        }
    }

    pub fn king(&self, color: Color) -> Square {
        self.bitboards[color][PieceType::King]
                                                                    .squares()
                                                                    .pop()
                                                                    .expect("King not found")
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

    fn update_castling_rights(&mut self, from: Square,
            to: Square,
            from_piece: Piece,
            captured_piece: Option<Piece>,) {

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

    fn make_enpassant_move(&mut self, piece_move: PieceMove){
        let pawn = self.piece_on_square[piece_move.from].expect("Expected pawn");
        let capture_square = match pawn.color {
            Color::White => {
                self.remove_piece(pawn, piece_move.from);
                self.add_piece(pawn, piece_move.to);
                Square::from_index(piece_move.to.index() - BOARDWIDTH)
                
            },
            Color::Black => {
                self.remove_piece(pawn, piece_move.from);
                self.add_piece(pawn, piece_move.to);
                Square::from_index(piece_move.to.index() + BOARDWIDTH)
            }
        };
        let captured_pawn = self.piece_on_square[capture_square].expect("Expected captured pawn");
        debug_assert_eq!(captured_pawn.piece_type, PieceType::Pawn);
        debug_assert_eq!(captured_pawn.color, color.opposite());
        self.remove_piece(captured_pawn, capture_square);
}


    fn undo_castle_move(&mut self, last_move: LogEntry) {
        let king = self.piece_on_square[last_move.to]
            .expect("Expected king on destination");

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

        self.remove_piece(
            Piece::new(PieceType::Rook, king.color),
            rook_old_square,
        );

        self.add_piece(
            Piece::new(PieceType::Rook, king.color),
            rook_new_square,
        );
    }
    fn undo_enpassant(&mut self, last_move: LogEntry) {
        let pawn = self.piece_on_square[last_move.to]
            .expect("Expected pawn on destination");

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
