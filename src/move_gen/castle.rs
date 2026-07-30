use crate::board::Board;
use crate::move_gen::MoveGenerator;
use crate::piece::{Color, MoveKind, PieceMove, PieceMoveList};
use crate::square::Square;
impl MoveGenerator {
    pub fn generate_castling(moves: &mut PieceMoveList, board: &Board, color: Color) {
        match color {
            Color::White => {
                if board.castling_rights.white_kingside
                    && board.empty.is_set(Square::F1)
                    && board.empty.is_set(Square::G1)
                {
                    moves.push(PieceMove::new(Square::E1, Square::G1, MoveKind::KingCastle))
                }
                if board.castling_rights.white_queenside
                    && board.empty.is_set(Square::B1)
                    && board.empty.is_set(Square::C1)
                    && board.empty.is_set(Square::D1)
                {
                    moves.push(PieceMove::new(
                        Square::E1,
                        Square::C1,
                        MoveKind::QueenCastle,
                    ))
                }
            }
            Color::Black => {
                if board.castling_rights.black_kingside
                    && board.empty.is_set(Square::F8)
                    && board.empty.is_set(Square::G8)
                {
                    moves.push(PieceMove::new(Square::E8, Square::G8, MoveKind::KingCastle))
                }
                if board.castling_rights.black_queenside
                    && board.empty.is_set(Square::B8)
                    && board.empty.is_set(Square::C8)
                    && board.empty.is_set(Square::D8)
                {
                    moves.push(PieceMove::new(
                        Square::E8,
                        Square::C8,
                        MoveKind::QueenCastle,
                    ))
                }
            }
        }
    }
}
