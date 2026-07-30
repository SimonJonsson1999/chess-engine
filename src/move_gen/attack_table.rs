use crate::bitboard::BitBoard;
use crate::piece::Color;
use crate::square::Square;
use std::ops::Index;
use crate::move_gen::constants::{KNIGHT_DIRECTIONS, KING_DIRECTIONS};
pub struct AttackTable([BitBoard; 64]);

impl AttackTable {
    pub const fn new(bitboards: [BitBoard; 64]) -> Self {
        Self(bitboards)
    }
}
impl Index<Square> for AttackTable {
    type Output = BitBoard;

    fn index(&self, square: Square) -> &Self::Output {
        &self.0[square as usize]
    }
}


const fn knight_attacks_from(square: Square) -> BitBoard {
        let mut attacks = BitBoard(0);

        // Extrac the rank and file from the square the knight is located on
        let rank = square.rank() as i8;
        let file = square.file() as i8;

        // Loop through all directions the knight can jump and calculate new rank and file indexes
        let mut i = 0;
        while i < KNIGHT_DIRECTIONS.len() {
            let (rank_offset, file_offset) = KNIGHT_DIRECTIONS[i];

            let target_rank = rank + rank_offset;
            let target_file = file + file_offset;

            let Some(target_square) = Square::try_from_rank_file(target_rank, target_file) else {
                // if square outside board, go to next direction
                i += 1;
                continue;
            };
            attacks.set(target_square);

            i += 1;
        }
        attacks
    }

const fn generate_knight_bitboards() -> AttackTable {
    let mut bitboards = [BitBoard(0); 64];
    let mut i: u8 = 0;
    while i < 64 {
        let square = Square::from_index(i);
        let knight_attacks: BitBoard = knight_attacks_from(square);
        bitboards[square.index() as usize] = knight_attacks;
        i += 1;
    }
    AttackTable::new(bitboards)
}

pub const KNIGHT_ATTACKS: AttackTable = generate_knight_bitboards();

const fn king_attacks_from(square: Square) -> BitBoard {
        let mut attacks = BitBoard(0);

        // Extract the rank and file from the square the king is located on.
        let rank = square.rank() as i8;
        let file = square.file() as i8;

        // Loop through all directions the king can move.
        let mut i = 0;
        while i < KING_DIRECTIONS.len() {
            let (rank_offset, file_offset) = KING_DIRECTIONS[i];

            let target_rank = rank + rank_offset;
            let target_file = file + file_offset;

            let Some(target_square) = Square::try_from_rank_file(target_rank, target_file) else {
                // if square outside board, go to next direction
                i += 1;
                continue;
            };
            attacks.set(target_square);
            i += 1;
        }
        attacks
    }

const fn generate_king_bitboards() -> AttackTable {
    let mut bitboards = [BitBoard(0); 64];
    let mut i: u8 = 0;
    while i < 64 {
        let square = Square::from_index(i);
        let king_attacks: BitBoard = king_attacks_from(square);
        bitboards[square.index() as usize] = king_attacks;
        i += 1;
    }
    AttackTable::new(bitboards)
}

pub const KING_ATTACKS: AttackTable = generate_king_bitboards();

const fn generate_pawn_attack_bitboards(color: Color) -> AttackTable {
    let mut attacks = [BitBoard(0); 64];

    let mut index = 0;
    while index < 64 {
        let square = Square::from_index(index);

        let rank = square.rank() as i8;
        let file = square.file() as i8;

        match color {
            Color::White => {
                if let Some(target) = Square::try_from_rank_file(rank + 1, file - 1) {
                    attacks[index as usize].set(target);
                }

                if let Some(target) = Square::try_from_rank_file(rank + 1, file + 1) {
                    attacks[index as usize].set(target);
                }
            }

            Color::Black => {
                if let Some(target) = Square::try_from_rank_file(rank - 1, file - 1) {
                    attacks[index as usize].set(target);
                }

                if let Some(target) = Square::try_from_rank_file(rank - 1, file + 1) {
                    attacks[index as usize].set(target);
                }
            }
        }

        index += 1;
    }

    AttackTable::new(attacks)
}

pub(crate) const WHITE_PAWN_ATTACKS: AttackTable = generate_pawn_attack_bitboards(Color::White);
pub(crate) const BLACK_PAWN_ATTACKS: AttackTable = generate_pawn_attack_bitboards(Color::Black);
