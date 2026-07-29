use std::println;

use crate::bb;
use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::log::Move;
use crate::piece::{Color, MoveKind, PieceMove, PieceType};
use crate::square::Square;

// Define of constans used for move generation
pub const RANK2: BitBoard = bb!(A2, B2, C2, D2, E2, F2, G2, H2);
pub const RANK4: BitBoard = bb!(A4, B4, C4, D4, E4, F4, G4, H4);
pub const RANK5: BitBoard = bb!(A5, B5, C5, D5, E5, F5, G5, H5);
pub const RANK7: BitBoard = bb!(A7, B7, C7, D7, E7, F7, G7, H7);
pub const FILEA: BitBoard = bb!(A1, A2, A3, A4, A5, A6, A7, A8);
pub const FILEH: BitBoard = bb!(H1, H2, H3, H4, H5, H6, H7, H8);
pub const BOARDWIDTH: u8 = 8;
pub const KNIGHT_DIRECTIONS: [(i8, i8); 8] = [
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
];
pub const KING_DIRECTIONS: [(i8, i8); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

const PROMOTION_PIECES: [PieceType; 4] = [
    PieceType::Queen,
    PieceType::Rook,
    PieceType::Bishop,
    PieceType::Knight,
];

pub struct MoveGenerator {}
impl MoveGenerator {
    pub fn generate_all_moves(board: &Board, color: Color) -> Vec<PieceMove> {
        let mut possible_moves = Vec::<PieceMove>::new();
        MoveGenerator::generate_pawn_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_knight_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_king_moves(&mut possible_moves, board, color);
        for possible_move in &possible_moves {
            println!("{}", possible_move);
        }
        possible_moves
    }
    #[inline]
    fn enemy_pieces(board: &Board, color: Color) -> BitBoard {
        match color {
            Color::White => board.black,
            Color::Black => board.white,
        }
    }
    // Pawn moves
    #[inline]
    fn pawn_from_square(to: Square, color: Color, offset: u8) -> Square {
        match color {
            Color::White => to - offset,
            Color::Black => to + offset,
        }
    }

    #[inline]
    fn pawn_step(pawns: BitBoard, color: Color, offset: u8) -> BitBoard {
        match color {
            Color::White => pawns << offset,
            Color::Black => pawns >> offset,
        }
    }

    #[inline]
    fn push_pawn_moves(
        moves: &mut Vec<PieceMove>,
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
    fn push_promotion_moves(
        moves: &mut Vec<PieceMove>,
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

    fn generate_pawn_moves(possible_moves: &mut Vec<PieceMove>, board: &Board, color: Color) {
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

    fn generate_single_push_pawn_moves(board: &Board, color: Color, moves: &mut Vec<PieceMove>) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let destinations = match color {
            Color::White => MoveGenerator::pawn_step(bb & !RANK7, color, BOARDWIDTH) & board.empty,
            Color::Black => MoveGenerator::pawn_step(bb & !RANK2, color, BOARDWIDTH) & board.empty,
        };
        MoveGenerator::push_pawn_moves(moves, destinations, color, BOARDWIDTH, MoveKind::Quiet);
    }

    fn generate_double_push_pawn_moves(board: &Board, color: Color, moves: &mut Vec<PieceMove>) {
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

    fn generate_left_capture_pawn_moves(board: &Board, color: Color, moves: &mut Vec<PieceMove>) {
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

    fn generate_right_capture_pawn_moves(board: &Board, color: Color, moves: &mut Vec<PieceMove>) {
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

    fn generate_promotion_pawn_moves(board: &Board, color: Color, moves: &mut Vec<PieceMove>) {
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
        moves: &mut Vec<PieceMove>,
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
        moves: &mut Vec<PieceMove>,
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

    fn generate_enpassant_left_moves(board: &Board, color: Color, moves: &mut Vec<PieceMove>) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let Some(enpassant_sq) = board.enpassant else {
            return;
        };
        let enpassant_bb = BitBoard::from_square(enpassant_sq);
        let destinations = match color {
            Color::White => {
                MoveGenerator::pawn_step(bb & (!FILEA), color, BOARDWIDTH - 1u8) & enpassant_bb
            }
            Color::Black => {
                MoveGenerator::pawn_step(bb & (!FILEH), color, BOARDWIDTH - 1u8) & enpassant_bb
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

    fn generate_enpassant_right_moves(board: &Board, color: Color, moves: &mut Vec<PieceMove>) {
        let bb = board.bitboards[color][PieceType::Pawn];
        let Some(enpassant_sq) = board.enpassant else {
            return;
        };
        let enpassant_bb = BitBoard::from_square(enpassant_sq);
        let destinations = match color {
            Color::White => {
                MoveGenerator::pawn_step(bb & (!FILEH), color, BOARDWIDTH + 1u8) & enpassant_bb
            }
            Color::Black => {
                MoveGenerator::pawn_step(bb & (!FILEA), color, BOARDWIDTH + 1u8) & enpassant_bb
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

    // Rook Moves

    // Knight moves
    fn knight_attacks_from(square: Square) -> BitBoard {
        let mut attacks = BitBoard(0);

        // Extrac the rank and file from the square the knight is located on
        let rank = square.rank() as i8;
        let file = square.file() as i8;
        
        // Loop through all directions the knight can jump and calculate new rank and file indexes
        for (rank_offset, file_offset) in KNIGHT_DIRECTIONS {
            let target_rank = rank + rank_offset;
            let target_file = file + file_offset;
            
            // Check that the new indexes did not wrap around, if they did it's not a valid move
            if !(0..8).contains(&target_rank) || !(0..8).contains(&target_file) {
                continue;
            }
            
            // Calculate the index of the square from the new rank and file indexes
            let target_index = target_rank * (BOARDWIDTH as i8) + target_file;
            let target_square = Square::from_index(target_index as u8);

            attacks.set(target_square);
        }

        attacks
    }

    fn generate_knight_moves(moves: &mut Vec<PieceMove>, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        for from in board.bitboards[color][PieceType::Knight].squares() {
            let attacks = MoveGenerator::knight_attacks_from(from);
            
            let empty_destinations = attacks & board.empty;
            for square in empty_destinations.squares() {
                moves.push(PieceMove::new(from, square, MoveKind::Quiet));
            }
            let enemy_destinations = attacks & enemies;
            for square in enemy_destinations.squares() {
                moves.push(PieceMove::new(from, square, MoveKind::Capture));
            }
        }
    }

    // King moves
    fn king_attacks_from(square: Square) -> BitBoard {
        let mut attacks = BitBoard(0);

        // Extrac the rank and file from the square the king is located on
        let rank = square.rank() as i8;
        let file = square.file() as i8;

        // Loop through all directions the King can go and calculate new rank and file indexes
        for (rank_offset, file_offset) in KING_DIRECTIONS {
            let target_rank = rank + rank_offset;
            let target_file = file + file_offset;
            
            // Check that the new indexes did not wrap around, if they did it's not a valid move
            if !(0..8).contains(&target_rank) || !(0..8).contains(&target_file) {
                continue;
            }
            
            // Calculate the index of the square from the new rank and file indexes
            let target_index = target_rank * (BOARDWIDTH as i8) + target_file;
            let target_square = Square::from_index(target_index as u8);

            attacks.set(target_square);
        }
        attacks
    }

    fn generate_king_moves(moves: &mut Vec<PieceMove>, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let square = board.bitboards[color][PieceType::King]
                                                                    .squares()
                                                                    .pop()
                                                                    .expect("King not found");
        let attacks = MoveGenerator::king_attacks_from(square);
        let empty_destinations = attacks & board.empty;
            for attack_square in empty_destinations.squares() {
                moves.push(PieceMove::new(square, attack_square, MoveKind::Quiet));
            }
            let enemy_destinations = attacks & enemies;
            for attack_square in enemy_destinations.squares() {
                moves.push(PieceMove::new(square, attack_square, MoveKind::Capture));
            }
}

}