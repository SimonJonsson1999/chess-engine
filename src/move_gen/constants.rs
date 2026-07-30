use crate::bb;
use crate::bitboard::BitBoard;
use crate::piece::PieceType;
// Define of constans used for move generation
pub(crate) const RANK2: BitBoard = bb!(A2, B2, C2, D2, E2, F2, G2, H2);
pub(crate) const RANK7: BitBoard = bb!(A7, B7, C7, D7, E7, F7, G7, H7);
pub(crate) const FILEA: BitBoard = bb!(A1, A2, A3, A4, A5, A6, A7, A8);
pub(crate) const FILEH: BitBoard = bb!(H1, H2, H3, H4, H5, H6, H7, H8);
pub const BOARDWIDTH: u8 = 8;
pub(crate) const KNIGHT_DIRECTIONS: [(i8, i8); 8] = [
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
];
pub(crate) const KING_DIRECTIONS: [(i8, i8); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

pub(crate) const STRAIGHT_DIRECTIONS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub(crate) const DIAG_DIRECTIONS: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

pub(crate) const PROMOTION_PIECES: [PieceType; 4] = [
    PieceType::Queen,
    PieceType::Rook,
    PieceType::Bishop,
    PieceType::Knight,
];
