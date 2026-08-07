use crate::board::Board;
use crate::board::piece::{Color, PieceType};
use crate::evaluation::positions::{
    BISHOP_PST, KING_MIDDLE_PST, KNIGHT_PST, PAWN_PST, QUEEN_PST, ROOK_PST,
};
pub struct BoardEvaluation;

// Consider using const array like this in future
// const PIECE_VALUES: [i32; 6] = [
//     100, // Pawn
//     320, // Knight
//     330, // Bishop
//     500, // Rook
//     900, // Queen
//     0,   // King
// ];

pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> i32;
}
impl BoardEvaluation {
    fn material(board: &Board) -> i32 {
        let mut sum: i32 = 0;

        let pieces = [
            PieceType::Pawn,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
        ];
        let colors = [Color::White, Color::Black];
        for color in colors {
            let modifier: i32 = match color {
                Color::White => 1,
                Color::Black => -1,
            };
            for piece_type in pieces {
                let bitboard = board.bitboards[color][piece_type];
                let piece_count = bitboard.count() as i32;
                sum += piece_count * modifier * piece_type.value();
            }
        }
        sum
    }

    fn piece_positions(board: &Board) -> i32 {
        let mut sum: i32 = 0;

        let pieces = [
            PieceType::Pawn,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::King,
        ];
        let colors = [Color::White, Color::Black];
        for color in colors {
            let modifier: i32 = match color {
                Color::White => 1,
                Color::Black => -1,
            };
            for piece_type in pieces {
                let bitboard = board.bitboards[color][piece_type];
                for square in bitboard.squares() {
                    let index = match color {
                        Color::White => square.index() as usize,
                        Color::Black => square.flip().index() as usize,
                    };
                    let value = match piece_type {
                        PieceType::Pawn => PAWN_PST[index],
                        PieceType::Knight => KNIGHT_PST[index],
                        PieceType::Bishop => BISHOP_PST[index],
                        PieceType::Rook => ROOK_PST[index],
                        PieceType::Queen => QUEEN_PST[index],
                        PieceType::King => KING_MIDDLE_PST[index],
                    };
                    sum += modifier * value;
                }
            }
        }
        sum
    }

    fn tempo(board: &Board) -> i32 {
        let mut score = 0;
        match board.turn {
            Color::White => score = 10,
            Color::Black => score = 10,
        }
        score
    }
}
impl Evaluator for BoardEvaluation {
    fn evaluate(&self, board: &Board) -> i32 {
        Self::material(board) + Self::piece_positions(board) + Self::tempo(board)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval_start_position() {
        let board = Board::default();
        let evaluator = BoardEvaluation {};
        assert_eq!(evaluator.evaluate(&board), 10);
    }
}
