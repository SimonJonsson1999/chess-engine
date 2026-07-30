use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::move_gen::MoveGenerator;
use crate::move_gen::attack_table::KNIGHT_ATTACKS;
use crate::move_gen::constants::KNIGHT_DIRECTIONS;
use crate::piece::{Color, PieceMoveList, PieceType};
use crate::square::Square;

impl MoveGenerator {
    // Knight moves
    pub(crate) const fn knight_attacks_from(square: Square) -> BitBoard {
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

    pub(crate) fn generate_knight_moves(moves: &mut PieceMoveList, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let knight_bitboard: BitBoard = board.bitboards[color][PieceType::Knight];
        for from_sq in knight_bitboard.squares() {
            let attacks = KNIGHT_ATTACKS[from_sq];
            MoveGenerator::push_moves(moves, from_sq, attacks, board.empty, enemies);
        }
    }
}
