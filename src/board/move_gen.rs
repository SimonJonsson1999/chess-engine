use std::println;
use std::ops::Index;
use crate::bb;
use crate::bitboard::BitBoard;
use crate::board::Board;
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

pub const ROOK_DIRECTIONS: [(i8, i8); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

pub const BISHOP_DIRECTIONS: [(i8, i8); 4] = [
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

pub const PROMOTION_PIECES: [PieceType; 4] = [
    PieceType::Queen,
    PieceType::Rook,
    PieceType::Bishop,
    PieceType::Knight,
];



// Structure to hold the 64 bitboards for attacks
// Needed so we can index using square, removing annoying 'square.index() as usize' everywhere
// My hope is that this gets optimized away at compile time so no runtime overhead,
// just improving readability
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

pub struct MoveGenerator {}
impl MoveGenerator {
    pub fn generate_all_moves(board: &Board, color: Color) -> Vec<PieceMove> {
        let mut possible_moves = Vec::<PieceMove>::new();
        MoveGenerator::generate_pawn_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_knight_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_king_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_rook_moves(&mut possible_moves, board, color);
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

    // TODO generate these moves using magic bitboards and access for each square using index. 

    // Rook Moves

    fn rook_attacks_from_sq(from_sq: Square, board: &Board, color: Color) -> BitBoard {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let mut attacks = BitBoard(0);
        let rank = from_sq.rank() as i8;
        let file = from_sq.file() as i8;
        let mut i = 0;
        // Step in each direction and check if the aquare is empty or occupied
        // update the attack bb accordingly and once occupied square or end of
        // board is found, go to next direction
        while i < ROOK_DIRECTIONS.len() {
            let (rank_direction, file_direction) = ROOK_DIRECTIONS[i];
            let mut j: i8 = 1;
            // Step in direction j steps
            while j < (BOARDWIDTH as i8){
                // Calculate new rank and file afer stepping
                let new_rank = rank + j*rank_direction;
                let new_file = file + j*file_direction;
                // Check if out of bounds after stepping in direction
                // if so go to next direction
                if new_rank < 0
                    || new_rank >= 8
                    || new_file < 0
                    || new_file >= 8
                {
                    break
                }

                // Create the target square and check if it is occupied or not
                // and if occupied ny enemy or friendly piece.
                let target_index = new_rank * BOARDWIDTH as i8 + new_file;
                let target_square = Square::from_index(target_index as u8);
                let bb = BitBoard::from_square(target_square);
                if board.piece_on_square[target_square].is_none(){
                    // Empty square detected, possible to move to
                    // Keep looking in this direction
                    attacks.set(target_square);
                    j += 1;
                    continue;
                }
                else if (bb & enemies).is_non_empty() {
                    // Enemy piece detected, set square as possible to move to,
                    // but do not keep searching in this direction (blocked)
                    attacks.set(target_square);
                    break;  
                }
                else {
                    // Friendly piece blocks, go to next direction
                    break;
                }
                }
        i += 1;         
        }
        

    attacks 
    }
    




    fn generate_rook_moves(moves: &mut Vec<PieceMove>, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let rook_bitboard = board.bitboards[color][PieceType::Rook];
        for from_sq in  rook_bitboard.squares(){
            let attacks = MoveGenerator::rook_attacks_from_sq(from_sq, board, color);
            

            // TODO Double check that this is correct, was copied from knight function.
            let empty_destinations = attacks & board.empty;
            for to_square in empty_destinations.squares() {
                moves.push(PieceMove::new(from_sq, to_square, MoveKind::Quiet));
            }
            let enemy_destinations = attacks & enemies;
            for to_square in enemy_destinations.squares() {
                moves.push(PieceMove::new(from_sq, to_square, MoveKind::Capture));
            }
        } 

    }

    // Bishop moves

    // Queen moves

    // Knight moves
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

                // Skip moves that would leave the board.
                if target_rank < 0
                    || target_rank >= 8
                    || target_file < 0
                    || target_file >= 8
                {
                    i += 1;
                    continue;
                }
                
                // Calculate the index of the square from the new rank and file indexes
                let target_index = target_rank * BOARDWIDTH as i8 + target_file;
                let target_square = Square::from_index(target_index as u8);

                attacks.set(target_square);

                i += 1;
            }
        attacks
        }

        

    fn generate_knight_moves(moves: &mut Vec<PieceMove>, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let knight_bitboard: BitBoard = board.bitboards[color][PieceType::Knight];
        for from_sq in  knight_bitboard.squares(){
            let attacks = KNIGHT_ATTACKS[from_sq];
            
            let empty_destinations = attacks & board.empty;
            for to_square in empty_destinations.squares() {
                moves.push(PieceMove::new(from_sq, to_square, MoveKind::Quiet));
            }
            let enemy_destinations = attacks & enemies;
            for to_square in enemy_destinations.squares() {
                moves.push(PieceMove::new(from_sq, to_square, MoveKind::Capture));
            }
        }
    }
    
    // King moves
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

            // Skip moves that would leave the board.
            if target_rank >= 0
                && target_rank < 8
                && target_file >= 0
                && target_file < 8
            {
                let target_index = target_rank * BOARDWIDTH as i8 + target_file;
                attacks.set(Square::from_index(target_index as u8));
            }

            i += 1;
        }
        attacks
    }

    fn generate_king_moves(moves: &mut Vec<PieceMove>, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let square = board.bitboards[color][PieceType::King]
                                                                    .squares()
                                                                    .pop()
                                                                    .expect("King not found");
        let attacks = KING_ATTACKS[square];
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