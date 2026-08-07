use crate::board::Board;
use crate::board::bitboard::BitBoard;
use crate::board::piece::{Color, MoveKind, PieceMove, PieceMoveList, PieceType};
use crate::board::square::Square;
use crate::move_gen::MoveGenerator;
use crate::move_gen::attack_table::{
    BLACK_PAWN_ATTACKS, KING_ATTACKS, KNIGHT_ATTACKS, WHITE_PAWN_ATTACKS,
};
use crate::move_gen::constants::BOARDWIDTH;
use crate::move_gen::constants::{DIAG_DIRECTIONS, STRAIGHT_DIRECTIONS};

impl MoveGenerator {
    pub(crate) fn ray_attacks_from_sq(
        from_sq: Square,
        occupied: BitBoard,
        directions: &[(i8, i8)],
    ) -> BitBoard {
        let mut attacks = BitBoard(0);
        let rank = from_sq.rank() as i8;
        let file = from_sq.file() as i8;
        let mut i = 0;
        // Step in each direction and check if the aquare is empty or occupied
        // update the attack bb accordingly and once occupied square or end of
        // board is found, go to next direction
        while i < directions.len() {
            let (rank_direction, file_direction) = directions[i];
            let mut j: i8 = 1;
            // Step in direction j steps
            while j < (BOARDWIDTH as i8) {
                // Calculate new rank and file afer stepping
                let new_rank = rank + j * rank_direction;
                let new_file = file + j * file_direction;
                let Some(target_square) = Square::try_from_rank_file(new_rank, new_file) else {
                    // if square outside board, go to next direction
                    break;
                };
                if (occupied & target_square).is_empty() {
                    // Empty square detected, possible to move to
                    // Keep looking in this direction
                    attacks.set(target_square);
                    j += 1;
                    continue;
                } else {
                    // Piece detected, set square as possible to move to,
                    // but do not keep searching in this direction (blocked)
                    attacks.set(target_square);
                    break;
                }
            }
            i += 1;
        }
        attacks
    }

    #[inline]
    pub(crate) fn enemy_pieces(board: &Board, color: Color) -> BitBoard {
        match color {
            Color::White => board.black,
            Color::Black => board.white,
        }
    }

    pub(crate) fn push_moves(
        moves: &mut PieceMoveList,
        from: Square,
        attacks: BitBoard,
        empty: BitBoard,
        enemies: BitBoard,
    ) {
        let quiets = attacks & empty;
        for to in quiets.squares() {
            moves.push(PieceMove::new(from, to, MoveKind::Quiet));
        }

        let captures = attacks & enemies;
        for to in captures.squares() {
            moves.push(PieceMove::new(from, to, MoveKind::Capture));
        }
    }

    pub(crate) fn is_square_attacked(board: &Board, square: Square, by: Color) -> bool {
        let occupied = !board.empty;
        let diagonal = MoveGenerator::ray_attacks_from_sq(square, occupied, &DIAG_DIRECTIONS);
        let straight = MoveGenerator::ray_attacks_from_sq(square, occupied, &STRAIGHT_DIRECTIONS);
        let enemy_queens = board.bitboards[by][PieceType::Queen];
        let enemy_rooks = board.bitboards[by][PieceType::Rook];
        let enemy_bishops = board.bitboards[by][PieceType::Bishop];
        let enemy_pawns = board.bitboards[by][PieceType::Pawn];
        let pawn_attackers = match by {
            Color::White => BLACK_PAWN_ATTACKS[square],
            Color::Black => WHITE_PAWN_ATTACKS[square],
        };

        (KNIGHT_ATTACKS[square] & board.bitboards[by][PieceType::Knight]).is_non_empty()
            || (KING_ATTACKS[square] & board.bitboards[by][PieceType::King]).is_non_empty()
            || (diagonal & (enemy_bishops | enemy_queens)).is_non_empty()
            || (straight & (enemy_rooks | enemy_queens)).is_non_empty()
            || (pawn_attackers & enemy_pawns).is_non_empty()
    }
}
