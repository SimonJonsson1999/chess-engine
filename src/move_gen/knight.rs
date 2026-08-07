use crate::board::Board;
use crate::board::bitboard::BitBoard;
use crate::board::piece::{Color, PieceMoveList, PieceType};
use crate::move_gen::MoveGenerator;
use crate::move_gen::attack_table::KNIGHT_ATTACKS;

impl MoveGenerator {
    // Knight moves
    pub(crate) fn generate_knight_moves(moves: &mut PieceMoveList, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let knight_bitboard: BitBoard = board.bitboards[color][PieceType::Knight];
        for from_sq in knight_bitboard.squares() {
            let attacks = KNIGHT_ATTACKS[from_sq];
            MoveGenerator::push_moves(moves, from_sq, attacks, board.empty, enemies);
        }
    }
}
