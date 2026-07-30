use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::move_gen::MoveGenerator;
use crate::move_gen::constants::DIAG_DIRECTIONS;
use crate::piece::{Color, PieceMoveList, PieceType};
impl MoveGenerator {
    pub(crate) fn generate_diag_slider_moves(
        moves: &mut PieceMoveList,
        board: &Board,
        color: Color,
    ) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let occupied: BitBoard = !board.empty;
        let slider_bitboard =
            board.bitboards[color][PieceType::Queen] | board.bitboards[color][PieceType::Bishop];
        for from_sq in slider_bitboard.squares() {
            let attacks = MoveGenerator::ray_attacks_from_sq(from_sq, occupied, &DIAG_DIRECTIONS);
            MoveGenerator::push_moves(moves, from_sq, attacks, board.empty, enemies);
        }
    }
}
