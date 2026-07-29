use crate::square::Square;
use std::fmt;

// Each piece is represented using 8 bytes
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
impl PieceType {
    pub const fn idx(self) -> usize {
        self as usize
    }
}
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PieceType::Pawn => "P",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        };

        write!(f, "{s}")
    }
}

// The color of a piece is represented using 8 bits
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}
impl Color {
    pub const fn idx(self) -> usize {
        self as usize
    }
    pub const fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Color::White => "w",
            Color::Black => "b",
        };

        write!(f, "{s}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }
}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.color, self.piece_type)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PieceMove {
    pub from: Square,
    pub to: Square,
    pub kind: MoveKind,
}

impl PieceMove {
    pub fn new(from: Square, to: Square, kind: MoveKind) -> Self {
        PieceMove { from, to, kind }
    }
    pub const NULL: Self = Self {
        from: Square::A1,
        to: Square::A1,
        kind: MoveKind::Quiet,
    };

}

impl fmt::Display for PieceMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.from, self.to)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PieceMoveList {
    moves: [PieceMove; 256],
    len: u8,
}
impl PieceMoveList {

    pub const fn new() -> Self {
        Self {
            moves: [PieceMove::NULL; 256],
            len: 0,
        }
    }

    pub fn push(&mut self, mv: PieceMove) {
        if (self.len as usize) >= self.moves.len() {
        panic!("Overflow while pushing {}", mv);
    }
        self.moves[self.len as usize] = mv;
        self.len += 1;
    }
    pub fn pop(&mut self) -> PieceMove {
        debug_assert!(self.len as usize > 0);
        self.len -= 1;
        let latest_move = self.moves[(self.len()) as usize];
        latest_move
    }

    pub fn len(&self) -> u8 {
        self.len
    }

    pub fn iter(&self) -> impl Iterator<Item = &PieceMove> {
        self.moves[..self.len as usize].iter()
    }
} // impl PieceMoveList

impl FromIterator<PieceMove> for PieceMoveList {
    fn from_iter<T: IntoIterator<Item = PieceMove>>(iter: T) -> Self {
        let mut list = PieceMoveList::new();

        for mv in iter {
            list.push(mv);
        }

        list
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveKind {
    Quiet,
    Capture,
    DoublePawnPush,
    EnPassant,
    KingCastle,
    QueenCastle,
    Promotion(PieceType),
    PromotionCapture(PieceType),
}
