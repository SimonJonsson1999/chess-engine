use crate::board::Square;
use crate::board::piece::{Color, Piece, PieceType};
use std::ops::{BitAnd, BitOr, Index, IndexMut, Not, Shl, Shr};

#[macro_export]
macro_rules! bb {
    ($($sq:ident),* $(,)?) => {
        $crate::board::bitboard::BitBoard(
            0 $(| (1u64 << ($crate::board::Square::$sq as u8)))*
        )
    };
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BitBoard(pub u64);
impl BitBoard {
    #[inline]
    pub const fn from_square(square: Square) -> Self {
        BitBoard(1u64 << (square as u8))
    }
    pub const fn set(&mut self, square: Square) {
        self.0 |= 1u64 << (square as u8);
    }
    pub const fn clear(&mut self, square: Square) {
        self.0 &= !(1u64 << (square as u8))
    }
    pub const fn is_set(&self, square: Square) -> bool {
        let square_mask = 1u64 << (square as u8);
        (self.0 & square_mask) != 0
    }
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }
    pub const fn is_non_empty(&self) -> bool {
        self.0 != 0
    }
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
    pub fn squares(&self) -> Vec<Square> {
        let mut squares = Vec::new();
        let mut bb = self.0;

        while bb != 0 {
            let index = bb.trailing_zeros() as u8;
            squares.push(Square::from_index(index));
            bb &= bb - 1; // Clear the least significant set bit
        }
        squares
    }

    pub fn debug_grid(&self) -> String {
        let mut grid = String::from("    A B C D E F G H\n\n");

        for rank in (0..8).rev() {
            grid.push_str(&format!("{}   ", rank + 1));

            for file in 0..8 {
                let index = rank * 8 + file;
                let square = Square::from_index(index);
                let marker = if self.is_set(square) { '■' } else { '□' };

                grid.push(marker);

                if file < 7 {
                    grid.push(' ');
                }
            }

            if rank > 0 {
                grid.push_str("\n\n");
            }
        }

        grid
    }
}

impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug_grid())
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOr<Square> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, rhs: Square) -> Self::Output {
        BitBoard(self.0 | (1u64 << rhs.index()))
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAnd<Square> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, rhs: Square) -> Self::Output {
        BitBoard(self.0 & (1u64 << rhs.index()))
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl Shl<u8> for BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: u8) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl Shr<u8> for BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: u8) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

/// The six bitboards belonging to one color.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ColorBitBoards([BitBoard; 6]);

impl ColorBitBoards {
    pub fn new(bitboards: [BitBoard; 6]) -> Self {
        Self(bitboards)
    }
}

impl Index<PieceType> for ColorBitBoards {
    type Output = BitBoard;

    fn index(&self, piece: PieceType) -> &Self::Output {
        &self.0[piece as usize]
    }
}

impl IndexMut<PieceType> for ColorBitBoards {
    fn index_mut(&mut self, piece: PieceType) -> &mut Self::Output {
        &mut self.0[piece as usize]
    }
}

impl Default for ColorBitBoards {
    fn default() -> Self {
        Self([BitBoard(0); 6])
    }
}

/// All twelve piece bitboards.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BitBoards([ColorBitBoards; 2]);

impl BitBoards {
    pub fn new(data: [ColorBitBoards; 2]) -> Self {
        Self(data)
    }
}

impl Index<Color> for BitBoards {
    type Output = ColorBitBoards;

    fn index(&self, color: Color) -> &Self::Output {
        &self.0[color as usize]
    }
}

impl IndexMut<Color> for BitBoards {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        &mut self.0[color as usize]
    }
}

impl Index<Piece> for BitBoards {
    type Output = BitBoard;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self[piece.color][piece.piece_type]
    }
}

impl IndexMut<Piece> for BitBoards {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self[piece.color][piece.piece_type]
    }
}

impl Default for BitBoards {
    fn default() -> Self {
        Self([ColorBitBoards::default(); 2])
    }
}
