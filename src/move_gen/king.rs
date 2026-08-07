use crate::board::Board;
use crate::board::piece::{Color, PieceMoveList};
use crate::move_gen::MoveGenerator;
use crate::move_gen::attack_table::KING_ATTACKS;
impl MoveGenerator {
    // King moves

    pub(crate) fn generate_king_moves(moves: &mut PieceMoveList, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let square = board.king(color);
        let attacks = KING_ATTACKS[square];
        MoveGenerator::push_moves(moves, square, attacks, board.empty, enemies);
    }
}
