use crate::board::Board;
use crate::board::bitboard::BitBoard;
use crate::board::piece::{Color, MoveKind, Piece, PieceMove, PieceMoveList, PieceType};
use crate::board::square::Square;
use crate::move_gen::MoveGenerator;
use crate::move_gen::attack_table::{BLACK_PAWN_ATTACKS, WHITE_PAWN_ATTACKS};
use crate::move_gen::constants::{BOARDWIDTH, FILEA, FILEH, PROMOTION_PIECES, RANK2, RANK7};

impl MoveGenerator {
    pub(crate) fn has_legal_enpassant(board: &Board) -> bool {
        let Some(to) = board.enpassant else {
            return false;
        };
        let color = board.turn;
        let expected_rank = match color {
            Color::White => 5,
            Color::Black => 2,
        };
        if to.rank() != expected_rank || !board.empty.is_set(to) {
            return false;
        }

        let captured = Self::pawn_from_square(to, color, BOARDWIDTH);
        if board.piece_on_square[captured] != Some(Piece::new(PieceType::Pawn, color.opposite())) {
            return false;
        }
        let attackers = match color {
            Color::White => BLACK_PAWN_ATTACKS[to],
            Color::Black => WHITE_PAWN_ATTACKS[to],
        } & board.bitboards[color][PieceType::Pawn];

        for from in attackers.squares() {
            // Test king safety without making a move (hash updates also use this).
            let mut occupied = !board.empty;
            occupied.clear(from);
            occupied.clear(captured);
            occupied.set(to);
            let mut enemy_pawns = board.bitboards[color.opposite()][PieceType::Pawn];
            enemy_pawns.clear(captured);
            if !Self::is_square_attacked_with_occupancy(
                board,
                board.king(color),
                color.opposite(),
                occupied,
                enemy_pawns,
            ) {
                return true;
            }
        }
        false
    }

    #[inline]
    pub(crate) fn pawn_from_square(to: Square, color: Color, offset: u8) -> Square {
        match color {
            Color::White => to - offset,
            Color::Black => to + offset,
        }
    }

    #[inline]
    pub(crate) fn pawn_step(pawns: BitBoard, color: Color, offset: u8) -> BitBoard {
        match color {
            Color::White => pawns << offset,
            Color::Black => pawns >> offset,
        }
    }

    #[inline]
    pub(crate) fn push_pawn_moves(
        moves: &mut PieceMoveList,
        destinations: BitBoard,
        color: Color,
        offset: u8,
        kind: MoveKind,
    ) {
        for to in destinations.squares() {
            moves.push(PieceMove::new(
                MoveGenerator::pawn_from_square(to, color, offset),
                to,
                kind,
            ));
        }
    }

    #[inline]
    pub(crate) fn push_promotion_moves(
        moves: &mut PieceMoveList,
        destinations: BitBoard,
        color: Color,
        offset: u8,
        capture: bool,
    ) {
        for to in destinations.squares() {
            let from = MoveGenerator::pawn_from_square(to, color, offset);
            for piece_type in PROMOTION_PIECES {
                let kind = if capture {
                    MoveKind::PromotionCapture(piece_type)
                } else {
                    MoveKind::Promotion(piece_type)
                };
                moves.push(PieceMove::new(from, to, kind));
            }
        }
    }

    // Pawn Moves
    pub(crate) fn generate_pawn_moves(
        possible_moves: &mut PieceMoveList,
        board: &Board,
        color: Color,
    ) {
        MoveGenerator::generate_single_push_pawn_moves(board, color, possible_moves);
        MoveGenerator::generate_double_push_pawn_moves(board, color, possible_moves);
        MoveGenerator::generate_left_capture_pawn_moves(board, color, possible_moves);
        MoveGenerator::generate_right_capture_pawn_moves(board, color, possible_moves);
        MoveGenerator::generate_promotion_pawn_moves(board, color, possible_moves);
        MoveGenerator::generate_promotion_left_capture_pawn_moves(board, color, possible_moves);
        MoveGenerator::generate_promotion_right_capture_pawn_moves(board, color, possible_moves);
        MoveGenerator::generate_enpassant_left_moves(board, color, possible_moves);
        MoveGenerator::generate_enpassant_right_moves(board, color, possible_moves);
    }

    fn generate_single_push_pawn_moves(board: &Board, color: Color, moves: &mut PieceMoveList) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let destinations = match color {
            Color::White => MoveGenerator::pawn_step(bb & !RANK7, color, BOARDWIDTH) & board.empty,
            Color::Black => MoveGenerator::pawn_step(bb & !RANK2, color, BOARDWIDTH) & board.empty,
        };
        MoveGenerator::push_pawn_moves(moves, destinations, color, BOARDWIDTH, MoveKind::Quiet);
    }

    fn generate_double_push_pawn_moves(board: &Board, color: Color, moves: &mut PieceMoveList) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let destinations = match color {
            Color::White => {
                let single_pushes =
                    MoveGenerator::pawn_step(bb & RANK2, color, BOARDWIDTH) & board.empty;
                MoveGenerator::pawn_step(single_pushes, color, BOARDWIDTH) & board.empty
            }
            Color::Black => {
                let single_pushes =
                    MoveGenerator::pawn_step(bb & RANK7, color, BOARDWIDTH) & board.empty;
                MoveGenerator::pawn_step(single_pushes, color, BOARDWIDTH) & board.empty
            }
        };
        MoveGenerator::push_pawn_moves(
            moves,
            destinations,
            color,
            BOARDWIDTH * 2,
            MoveKind::DoublePawnPush,
        );
    }

    fn generate_left_capture_pawn_moves(board: &Board, color: Color, moves: &mut PieceMoveList) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let enemies = MoveGenerator::enemy_pieces(board, color);
        // To get squares 1 step forward and to the left we need to shift
        // the boardwidth - 1 and make sure we are not on the left edge to get wrapping.
        let destinations = match color {
            Color::White => {
                MoveGenerator::pawn_step(bb & !FILEA & !RANK7, color, BOARDWIDTH - 1u8) & enemies
            }
            Color::Black => {
                MoveGenerator::pawn_step(bb & !FILEH & !RANK2, color, BOARDWIDTH - 1u8) & enemies
            }
        };
        MoveGenerator::push_pawn_moves(
            moves,
            destinations,
            color,
            BOARDWIDTH - 1u8,
            MoveKind::Capture,
        );
    }

    fn generate_right_capture_pawn_moves(board: &Board, color: Color, moves: &mut PieceMoveList) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let enemies = MoveGenerator::enemy_pieces(board, color);
        // To get squares 1 step forward and to the right we need to shift
        // the boardwidth + 1 and make sure we are not on the right edge to get wrapping.
        let destinations = match color {
            Color::White => {
                MoveGenerator::pawn_step(bb & !FILEH & !RANK7, color, BOARDWIDTH + 1u8) & enemies
            }
            Color::Black => {
                MoveGenerator::pawn_step(bb & !FILEA & !RANK2, color, BOARDWIDTH + 1u8) & enemies
            }
        };
        MoveGenerator::push_pawn_moves(
            moves,
            destinations,
            color,
            BOARDWIDTH + 1u8,
            MoveKind::Capture,
        );
    }

    fn generate_promotion_pawn_moves(board: &Board, color: Color, moves: &mut PieceMoveList) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let destinations = match color {
            Color::White => MoveGenerator::pawn_step(bb & RANK7, color, BOARDWIDTH) & board.empty,
            Color::Black => MoveGenerator::pawn_step(bb & RANK2, color, BOARDWIDTH) & board.empty,
        };
        MoveGenerator::push_promotion_moves(moves, destinations, color, BOARDWIDTH, false);
    }

    fn generate_promotion_left_capture_pawn_moves(
        board: &Board,
        color: Color,
        moves: &mut PieceMoveList,
    ) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let destinations = match color {
            Color::White => {
                MoveGenerator::pawn_step(bb & RANK7 & !FILEA, color, BOARDWIDTH - 1u8) & enemies
            }
            Color::Black => {
                MoveGenerator::pawn_step(bb & RANK2 & !FILEH, color, BOARDWIDTH - 1u8) & enemies
            }
        };
        MoveGenerator::push_promotion_moves(moves, destinations, color, BOARDWIDTH - 1u8, true);
    }

    fn generate_promotion_right_capture_pawn_moves(
        board: &Board,
        color: Color,
        moves: &mut PieceMoveList,
    ) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let destinations = match color {
            Color::White => {
                MoveGenerator::pawn_step(bb & RANK7 & !FILEH, color, BOARDWIDTH + 1u8) & enemies
            }
            Color::Black => {
                MoveGenerator::pawn_step(bb & RANK2 & !FILEA, color, BOARDWIDTH + 1u8) & enemies
            }
        };
        MoveGenerator::push_promotion_moves(moves, destinations, color, BOARDWIDTH + 1u8, true);
    }

    fn generate_enpassant_left_moves(board: &Board, color: Color, moves: &mut PieceMoveList) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let Some(enpassant_sq) = board.enpassant else {
            return;
        };
        let destinations = match color {
            Color::White => {
                MoveGenerator::pawn_step(bb & (!FILEA), color, BOARDWIDTH - 1u8) & enpassant_sq
            }
            Color::Black => {
                MoveGenerator::pawn_step(bb & (!FILEH), color, BOARDWIDTH - 1u8) & enpassant_sq
            }
        };
        MoveGenerator::push_pawn_moves(
            moves,
            destinations,
            color,
            BOARDWIDTH - 1u8,
            MoveKind::EnPassant,
        );
    }

    fn generate_enpassant_right_moves(board: &Board, color: Color, moves: &mut PieceMoveList) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let Some(enpassant_sq) = board.enpassant else {
            return;
        };
        let destinations = match color {
            Color::White => {
                MoveGenerator::pawn_step(bb & (!FILEH), color, BOARDWIDTH + 1u8) & enpassant_sq
            }
            Color::Black => {
                MoveGenerator::pawn_step(bb & (!FILEA), color, BOARDWIDTH + 1u8) & enpassant_sq
            }
        };
        MoveGenerator::push_pawn_moves(
            moves,
            destinations,
            color,
            BOARDWIDTH + 1u8,
            MoveKind::EnPassant,
        );
    }
}
