use std::ops::Index;
use crate::bb;
use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::log::LogEntry;
use crate::piece::{Color, MoveKind, PieceMove, PieceMoveList, PieceType};
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

pub const STRAIGHT_DIRECTIONS: [(i8, i8); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

pub const DIAG_DIRECTIONS: [(i8, i8); 4] = [
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

const WHITE_PAWN_ATTACKS: AttackTable = generate_pawn_attack_bitboards(Color::White);
const BLACK_PAWN_ATTACKS: AttackTable = generate_pawn_attack_bitboards(Color::Black);

pub struct MoveGenerator {}
impl MoveGenerator {

    pub fn generate_valid_moves(board: &mut Board, color: Color) -> PieceMoveList {
        let valid_moves: PieceMoveList = MoveGenerator::generate_all_moves(board, color)
            .iter()
            .copied()
            .filter(|mv| MoveGenerator::legal_move(*mv, board, color))
            .collect();

        for valid_move in valid_moves.iter() {
            println!("{}", valid_move);
        }

        valid_moves
    }
    fn generate_all_moves(board: &Board, color: Color) -> PieceMoveList {
        let mut possible_moves = PieceMoveList::new();
        MoveGenerator::generate_pawn_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_knight_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_king_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_diag_slider_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_straight_slider_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_castling(&mut possible_moves, board, color);
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

    fn push_moves(
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
    #[inline]
    fn push_promotion_moves(
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

    fn ray_attacks_from_sq(from_sq: Square, occupied: BitBoard, directions: &[(i8, i8)]) -> BitBoard {
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
            while j < (BOARDWIDTH as i8){
                // Calculate new rank and file afer stepping
                let new_rank = rank + j*rank_direction;
                let new_file = file + j*file_direction;
                let Some(target_square) = Square::try_from_rank_file(new_rank, new_file) else {
                    // if square outside board, go to next direction
                    break;
                };
                if (occupied & target_square).is_empty(){
                    // Empty square detected, possible to move to
                    // Keep looking in this direction
                    attacks.set(target_square);
                    j += 1;
                    continue;
                }
                else {
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

    // Pawn Moves
    fn generate_pawn_moves(possible_moves: &mut PieceMoveList, board: &Board, color: Color) {
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

    // TODO generate these moves using magic bitboards and access for each square using index. 


    fn generate_diag_slider_moves(moves: &mut PieceMoveList, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let occupied: BitBoard = !board.empty;
        let slider_bitboard = board.bitboards[color][PieceType::Queen] |
                                board.bitboards[color][PieceType::Bishop];
        for from_sq in slider_bitboard.squares(){
            let attacks = MoveGenerator::ray_attacks_from_sq(from_sq, occupied, &DIAG_DIRECTIONS);
            MoveGenerator::push_moves(moves, from_sq, attacks, board.empty, enemies);
        } 
    }

    fn generate_straight_slider_moves(moves: &mut PieceMoveList, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let occupied: BitBoard = !board.empty;
        let straight_bitboard = board.bitboards[color][PieceType::Queen] |
                                board.bitboards[color][PieceType::Rook];
        for from_sq in straight_bitboard.squares(){
            let attacks = MoveGenerator::ray_attacks_from_sq(from_sq, occupied, &STRAIGHT_DIRECTIONS);
            MoveGenerator::push_moves(moves, from_sq, attacks, board.empty, enemies);
        } 
    }

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

        

    fn generate_knight_moves(moves: &mut PieceMoveList, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let knight_bitboard: BitBoard = board.bitboards[color][PieceType::Knight];
        for from_sq in  knight_bitboard.squares(){
            let attacks = KNIGHT_ATTACKS[from_sq];
            MoveGenerator::push_moves(moves, from_sq, attacks, board.empty, enemies);
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
        
    

    fn generate_king_moves(moves: &mut PieceMoveList, board: &Board, color: Color) {
        let enemies = MoveGenerator::enemy_pieces(board, color);
        let square = board.king(color);
        let attacks = KING_ATTACKS[square];
        MoveGenerator::push_moves(moves, square, attacks, board.empty, enemies);
    }



    pub fn is_square_attacked(
        board: &Board,
        square: Square,
        by: Color,
    ) -> bool {
            let occupied = !board.empty;
            let diagonal = MoveGenerator::ray_attacks_from_sq(
                                                square,
                                                occupied,
                                                &DIAG_DIRECTIONS,
                                        );
            let straight = MoveGenerator::ray_attacks_from_sq(
                                                square,
                                                occupied,
                                                &STRAIGHT_DIRECTIONS,
                                        );
            let enemy_queens = board.bitboards[by][PieceType::Queen];
            let enemy_rooks = board.bitboards[by][PieceType::Rook];
            let enemy_bishops = board.bitboards[by][PieceType::Bishop];
            let enemy_pawns = board.bitboards[by][PieceType::Pawn];
            let pawn_attackers = match by {
                Color::White => BLACK_PAWN_ATTACKS[square],
                Color::Black => WHITE_PAWN_ATTACKS[square],
            };


            (KNIGHT_ATTACKS[square] & board.bitboards[by][PieceType::Knight]).is_non_empty() ||
            (KING_ATTACKS[square] & board.bitboards[by][PieceType::King]).is_non_empty() ||
            (diagonal & (enemy_bishops | enemy_queens)).is_non_empty() ||
            (straight & (enemy_rooks | enemy_queens)).is_non_empty() ||
            (pawn_attackers & enemy_pawns).is_non_empty()
    }
    pub fn generate_castling(moves: &mut PieceMoveList, board: &Board, color: Color) {
        match color {
            Color::White => {
                if board.castling_rights.white_kingside && board.empty.is_set(Square::F1) && board.empty.is_set(Square::G1) {
                    moves.push(PieceMove::new(Square::E1, Square::G1, MoveKind::KingCastle))
                }
                if board.castling_rights.white_queenside && board.empty.is_set(Square::B1) && board.empty.is_set(Square::C1) && board.empty.is_set(Square::D1){
                    moves.push(PieceMove::new(Square::E1, Square::C1, MoveKind::QueenCastle))
                }
            },
            Color::Black => {
                if board.castling_rights.black_kingside && board.empty.is_set(Square::F8) && board.empty.is_set(Square::G8) {
                    moves.push(PieceMove::new(Square::E8, Square::G8, MoveKind::KingCastle))
                }
                if board.castling_rights.black_queenside && board.empty.is_set(Square::B8) && board.empty.is_set(Square::C8) && board.empty.is_set(Square::D8) {
                    moves.push(PieceMove::new(Square::E8, Square::C8, MoveKind::QueenCastle))
                }
            }
        }
    }

    pub fn legal_move(psuedo_legal_move: PieceMove, board: &mut Board, color: Color) -> bool {
        let opposite_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        // TODO check legal castling
        if psuedo_legal_move.kind == MoveKind::KingCastle {
            let (through, destination) = match color {
                                                            Color::White => (Square::F1, Square::G1),
                                                            Color::Black => (Square::F8, Square::G8),
                                                        };
            let legal = !MoveGenerator::is_square_attacked(board, through, opposite_color) &&
                                        !MoveGenerator::is_square_attacked(board, destination, opposite_color) &&
                                        !MoveGenerator::is_square_attacked(board, board.king(color), opposite_color);
            return legal
        }
        if psuedo_legal_move.kind == MoveKind::QueenCastle {
            let (through, destination) = match color {
                                                            Color::White => (Square::D1, Square::C1),
                                                            Color::Black => (Square::D8, Square::C8),
                                                        };
            let legal = !MoveGenerator::is_square_attacked(board, through, opposite_color) &&
                                        !MoveGenerator::is_square_attacked(board, destination, opposite_color) &&
                                        !MoveGenerator::is_square_attacked(board, board.king(color), opposite_color);
            return legal
            
        };
        board.move_piece(psuedo_legal_move);
        let legal = !MoveGenerator::is_square_attacked(board, board.king(color), opposite_color);
        board.undo();
        return legal
    }
}