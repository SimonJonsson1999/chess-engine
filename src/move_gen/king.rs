use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::move_gen::MoveGenerator;
use crate::move_gen::attack_table::KING_ATTACKS;
use crate::move_gen::constants::KING_DIRECTIONS;
use crate::piece::{Color, PieceMoveList};
use crate::square::Square;
impl MoveGenerator {
    // King moves
    pub(crate) const fn king_attacks_from(square: Square) -> BitBoard {
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

    pub(crate) fn generate_king_moves(moves: &mut PieceMoveList, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let square = board.king(color);
        let attacks = KING_ATTACKS[square];
        MoveGenerator::push_moves(moves, square, attacks, board.empty, enemies);
    }
}
