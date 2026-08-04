use crate::bitboard::{BitBoard, BitBoards};
use crate::board::CastlingRights;
use crate::log::MoveLog;
use crate::move_gen::MoveGenerator;
use crate::piece::{Color, Piece, PieceType};
use crate::square::{Square, SquareMap};
const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
// All pieces are represented using 6*2 bitboards, for the 6 different pieces
// for both colors
// Each bitboard is 64 bits, where each bit will represent if the piece is present or not
#[derive(PartialEq, Eq, Debug)]
pub enum GameState {
    Ongoing,
    Check,
    Checkmate,
    Stalemate,
    FiftyMoveRule,
}
#[derive(PartialEq, Eq, Debug)]
pub struct Board {
    pub bitboards: BitBoards,
    pub piece_on_square: SquareMap<Option<Piece>>,
    pub empty: BitBoard,
    pub white: BitBoard,
    pub black: BitBoard,
    pub(crate) move_log: MoveLog,
    pub enpassant: Option<Square>,
    pub castling_rights: CastlingRights,
    pub turn: Color,
    pub half_move: u8,
    pub full_move: u8,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            bitboards: BitBoards::default(),
            piece_on_square: SquareMap::new([None; 64]),
            empty: BitBoard(!0), // or BitBoard(!0)
            white: BitBoard(0),
            black: BitBoard(0),
            move_log: MoveLog::new(),
            enpassant: None,
            castling_rights: CastlingRights::new(),
            turn: Color::White,
            half_move: 0,
            full_move: 0,

        }
    }

    pub fn king(&self, color: Color) -> Square {
        self.bitboards[color][PieceType::King]
            .squares()
            .pop()
            .expect("King not found")
    }

    pub(crate) fn switch_turn(&mut self) {
        match self.turn {
            Color::White => self.turn = Color::Black,
            Color::Black => self.turn = Color::White,
        }
    }
    pub fn is_in_check(&self, color: Color) -> bool {
        MoveGenerator::is_square_attacked(self, self.king(color), color.opposite())
    }
    pub fn has_legal_moves(&mut self) -> bool {
        !MoveGenerator::generate_valid_moves(self).is_empty()
    }

    pub fn game_state(&mut self) -> GameState {
        if self.half_move >= 100 {
            return GameState::FiftyMoveRule;
        }

        let in_check = self.is_in_check(self.turn);
        let has_moves = self.has_legal_moves();

        match (in_check, has_moves) {
            (false, true) => GameState::Ongoing,
            (true,  true) => GameState::Check,
            (true,  false) => GameState::Checkmate,
            (false, false) => GameState::Stalemate,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen(STARTING_FEN)
    }
}

