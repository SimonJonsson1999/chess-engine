use crate::board::{Board, Color, PieceMove, Piece, CastlingRights, Square, MoveKind, PieceType};
struct ZobristKeys {
    pieces: [[[u64; 64]; 6]; 2],
    castling: [u64; 16],
    en_passant: [u64; 8],
    side_to_move: u64,
}
const ZOBRIST_KEYS: ZobristKeys = generate_zobrist_keys();

const fn generate_zobrist_keys() -> ZobristKeys {
    let mut state = 0x1234_5678_9ABC_DEF0;
    let mut pieces = [[[0u64; 64]; 6]; 2];
    let mut castling = [0u64; 16];
    let mut en_passant = [0u64; 8];

    // generate pieces using state
    let mut color = 0;
    while color < 2 {
        let mut piece = 0;
        while piece < 6 {
            let mut square = 0;
            while square < 64 {
                pieces[color][piece][square] = next_random(&mut state);
                square += 1;
            }
            piece += 1;
        }
        color += 1;
    }
    // generate castling using same state
    let mut castling_index = 0;
    while castling_index < 16 {
        castling[castling_index] = next_random(&mut state);
        castling_index += 1;
    }
    
    // generate en passant using same state
    let mut en_passant_index = 0;
    while en_passant_index < 8 {
        en_passant[en_passant_index] = next_random(&mut state);
        en_passant_index += 1;
    }

    // generate side-to-move using same state
    let side_to_move = next_random(&mut state);

    ZobristKeys {
        pieces,
        castling,
        en_passant,
        side_to_move,
    }
}

const fn next_random(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);

    let mut z = *state;

    z = (z ^ (z >> 30))
        .wrapping_mul(0xBF58476D1CE4E5B9);

    z = (z ^ (z >> 27))
        .wrapping_mul(0x94D049BB133111EB);

    z ^ (z >> 31)
}
pub struct Zobrist;

impl Zobrist {
    pub fn compute(board: &Board) -> u64 {
        let mut hash = 0u64;

        for (square, piece) in board.piece_on_square.occupied() {
            hash ^= ZOBRIST_KEYS.pieces
                [piece.color.idx()]
                [piece.piece_type.idx()]
                [square.index() as usize];
        }

        hash ^= ZOBRIST_KEYS.castling[board.castling_rights.index()];
        if let Some(square) = board.enpassant {
            hash ^= ZOBRIST_KEYS.en_passant[square.file() as usize];
        }
        if board.turn == Color::Black {
            hash ^= ZOBRIST_KEYS.side_to_move;
        }
        hash
    }

    pub fn update(
        hash: u64,
        piece_move: PieceMove,
        moved_piece: Piece,
        captured_piece: Option<Piece>,
        previous_castling_rights: CastlingRights,
        new_castling_rights: CastlingRights,
        previous_enpassant: Option<Square>,
        new_enpassant: Option<Square>,
    ) -> u64 {
        let mut hash = hash;

        let from = piece_move.from;
        let to = piece_move.to;

        // Pieces
        match piece_move.kind {
            MoveKind::Quiet | MoveKind::DoublePawnPush => {
                hash ^= ZOBRIST_KEYS.pieces
                    [moved_piece.color.idx()]
                    [moved_piece.piece_type.idx()]
                    [from.index() as usize];

                hash ^= ZOBRIST_KEYS.pieces
                    [moved_piece.color.idx()]
                    [moved_piece.piece_type.idx()]
                    [to.index() as usize];
            }

            MoveKind::Capture => {
                let captured = captured_piece
                    .expect("Capture must have a captured piece");

                // Remove moving piece from old square
                hash ^= ZOBRIST_KEYS.pieces
                    [moved_piece.color.idx()]
                    [moved_piece.piece_type.idx()]
                    [from.index() as usize];

                // Remove captured piece
                hash ^= ZOBRIST_KEYS.pieces
                    [captured.color.idx()]
                    [captured.piece_type.idx()]
                    [to.index() as usize];

                // Add moving piece to destination
                hash ^= ZOBRIST_KEYS.pieces
                    [moved_piece.color.idx()]
                    [moved_piece.piece_type.idx()]
                    [to.index() as usize];
            }

            MoveKind::Promotion(promotion_piece_type) => {
                // Remove pawn
                hash ^= ZOBRIST_KEYS.pieces
                    [moved_piece.color.idx()]
                    [PieceType::Pawn.idx()]
                    [from.index() as usize];

                // Add promoted piece
                hash ^= ZOBRIST_KEYS.pieces
                    [moved_piece.color.idx()]
                    [promotion_piece_type.idx()]
                    [to.index() as usize];
            }

            MoveKind::PromotionCapture(promotion_piece_type) => {
                let captured = captured_piece
                    .expect("Promotion capture must have a captured piece");

                // Remove pawn
                hash ^= ZOBRIST_KEYS.pieces
                    [moved_piece.color.idx()]
                    [PieceType::Pawn.idx()]
                    [from.index() as usize];

                // Remove captured piece
                hash ^= ZOBRIST_KEYS.pieces
                    [captured.color.idx()]
                    [captured.piece_type.idx()]
                    [to.index() as usize];

                // Add promoted piece
                hash ^= ZOBRIST_KEYS.pieces
                    [moved_piece.color.idx()]
                    [promotion_piece_type.idx()]
                    [to.index() as usize];
            }

            MoveKind::EnPassant => {
                let captured = captured_piece
                    .expect("En passant must have a captured piece");

                let captured_square = match moved_piece.color {
                    Color::White => Square::from_index(to.index() - 8),
                    Color::Black => Square::from_index(to.index() + 8),
                };

                // Remove moving pawn from old square
                hash ^= Self::piece_key(moved_piece, from);

                // Add moving pawn to new square
                hash ^= Self::piece_key(moved_piece, to);

                // Remove captured pawn
                hash ^= Self::piece_key(captured, captured_square);
            }

            MoveKind::KingCastle | MoveKind::QueenCastle => {
                let (rook_from, rook_to) = match (moved_piece.color, piece_move.kind) {
                    (Color::White, MoveKind::KingCastle) => {
                        (Square::H1, Square::F1)
                    }
                    (Color::White, MoveKind::QueenCastle) => {
                        (Square::A1, Square::D1)
                    }
                    (Color::Black, MoveKind::KingCastle) => {
                        (Square::H8, Square::F8)
                    }
                    (Color::Black, MoveKind::QueenCastle) => {
                        (Square::A8, Square::D8)
                    }
                    _ => unreachable!(),
                };

                let rook = Piece::new(PieceType::Rook, moved_piece.color);

                // King
                hash ^= Self::piece_key(moved_piece, from);
                hash ^= Self::piece_key(moved_piece, to);

                // Rook
                hash ^= Self::piece_key(rook, rook_from);
                hash ^= Self::piece_key(rook, rook_to);
            }
        }

        // Remove previous castling state
        hash ^= ZOBRIST_KEYS.castling[previous_castling_rights.index()];

        // Add new castling state
        hash ^= ZOBRIST_KEYS.castling[new_castling_rights.index()];

        // Remove previous en passant state
        if let Some(square) = previous_enpassant {
            hash ^= ZOBRIST_KEYS.en_passant[square.file() as usize];
        }

        // Add new en passant state
        if let Some(square) = new_enpassant {
            hash ^= ZOBRIST_KEYS.en_passant[square.file() as usize];
        }

        // Switch side to move
        hash ^= ZOBRIST_KEYS.side_to_move;

        hash
    }


    fn piece_key(piece: Piece, square: Square) -> u64 {
    ZOBRIST_KEYS.pieces
        [piece.color.idx()]
        [piece.piece_type.idx()]
        [square.index() as usize]
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{MoveKind, PieceMove, Square, CastlingRights};

    #[test]
    fn identical_boards_have_same_hash() {
        let board1 = Board::default();
        let board2 = Board::default();

        assert_eq!(
            Zobrist::compute(&board1),
            Zobrist::compute(&board2)
        );
    }

    #[test]
    fn different_positions_have_different_hashes() {
        let board1 = Board::default();
        let mut board2 = Board::default();

        board2.move_piece(PieceMove::new(
            Square::E2,
            Square::E4,
            MoveKind::Quiet,
        ));

        assert_ne!(
            Zobrist::compute(&board1),
            Zobrist::compute(&board2)
        );
    }

    #[test]
    fn undo_restores_hash() {
        let mut board = Board::default();

        let original_hash = Zobrist::compute(&board);

        board.move_piece(PieceMove::new(
            Square::E2,
            Square::E4,
            MoveKind::Quiet,
        ));

        assert_ne!(
            original_hash,
            Zobrist::compute(&board)
        );

        board.undo();

        assert_eq!(
            original_hash,
            Zobrist::compute(&board)
        );
    }
    #[test]
    fn different_castling_rights_have_different_hashes() {
        let board1 = Board::default();
        let mut board2 = Board::default();

        board2.castling_rights = CastlingRights::none();

        assert_ne!(
            Zobrist::compute(&board1),
            Zobrist::compute(&board2)
        );
    }

    fn assert_hash_correct(board: &Board) {
        assert_eq!(
            board.zobrist_hash,
            Zobrist::compute(board),
            "Incremental Zobrist hash does not match computed hash"
        );
    }

    #[test]
    fn initial_hash_is_correct() {
        let board = Board::default();

        assert_hash_correct(&board);
    }

    #[test]
    fn quiet_move_updates_hash() {
        let mut board = Board::default();
        let original_hash = board.zobrist_hash;

        board.move_piece(PieceMove::new(
            Square::G1,
            Square::F3,
            MoveKind::Quiet,
        ));

        assert_ne!(board.zobrist_hash, original_hash);
        assert_hash_correct(&board);
    }

    #[test]
    fn double_pawn_push_updates_hash() {
        let mut board = Board::default();

        board.move_piece(PieceMove::new(
            Square::E2,
            Square::E4,
            MoveKind::DoublePawnPush,
        ));

        assert_hash_correct(&board);
    }

    #[test]
    fn capture_updates_hash() {
        let mut board = Board::from_fen(
            "4k3/8/8/3p4/4P3/8/8/4K3 w - - 0 1"
        );

        board.move_piece(PieceMove::new(
            Square::E4,
            Square::D5,
            MoveKind::Capture,
        ));

        assert_hash_correct(&board);
    }

    #[test]
    fn undo_restores_original_hash() {
        let mut board = Board::default();

        let original_hash = board.zobrist_hash;

        board.move_piece(PieceMove::new(
            Square::G1,
            Square::F3,
            MoveKind::Quiet,
        ));

        assert_hash_correct(&board);
        assert_ne!(board.zobrist_hash, original_hash);

        board.undo();

        assert_hash_correct(&board);
        assert_eq!(board.zobrist_hash, original_hash);
    }

    #[test]
    fn multiple_moves_and_undos_restore_hashes() {
        let mut board = Board::default();

        let hash0 = board.zobrist_hash;

        board.move_piece(PieceMove::new(
            Square::E2,
            Square::E4,
            MoveKind::DoublePawnPush,
        ));
        let hash1 = board.zobrist_hash;
        assert_hash_correct(&board);

        board.move_piece(PieceMove::new(
            Square::E7,
            Square::E5,
            MoveKind::DoublePawnPush,
        ));
        let hash2 = board.zobrist_hash;
        assert_hash_correct(&board);

        board.move_piece(PieceMove::new(
            Square::G1,
            Square::F3,
            MoveKind::Quiet,
        ));
        assert_hash_correct(&board);

        board.undo();

        assert_hash_correct(&board);
        assert_eq!(board.zobrist_hash, hash2);

        board.undo();

        assert_hash_correct(&board);
        assert_eq!(board.zobrist_hash, hash1);

        board.undo();

        assert_hash_correct(&board);
        assert_eq!(board.zobrist_hash, hash0);
    }
}