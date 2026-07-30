use crate::bitboard::BitBoard;
use crate::move_gen::MoveGenerator;
use crate::piece::Color;
use crate::square::Square;
use std::ops::Index;

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

// Structure to hold the 64 bitboards for attacks
// Needed so we can index using square, removing annoying 'square.index() as usize' everywhere
// My hope is that this gets optimized away at compile time so no runtime overhead,
// just improving readability

// Generate a array of 64 bitboards for knight attacks
// indexed by the square.index()
const fn generate_knight_bitboards() -> AttackTable {
    let mut bitboards = [BitBoard(0); 64];
    let mut i: u8 = 0;
    while i < 64 {
        let square = Square::from_index(i);
        let knight_attacks: BitBoard = MoveGenerator::knight_attacks_from(square);
        bitboards[square.index() as usize] = knight_attacks;
        i += 1;
    }
    AttackTable::new(bitboards)
}

pub const KNIGHT_ATTACKS: AttackTable = generate_knight_bitboards();

// Generate a array of 64 bitboards for king attacks
// indexed by the square.index()
const fn generate_king_bitboards() -> AttackTable {
    let mut bitboards = [BitBoard(0); 64];
    let mut i: u8 = 0;
    while i < 64 {
        let square = Square::from_index(i);
        let king_attacks: BitBoard = MoveGenerator::king_attacks_from(square);
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
