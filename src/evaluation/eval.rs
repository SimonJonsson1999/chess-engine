use crate::board::Board;
use crate::piece::{PieceType, Color};
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


impl BoardEvaluation {
    pub fn evaluate(board: &Board) -> i32 {
        // Later other functions such as piece positioning etc can be added
        Self::material(board)
    }

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
                sum +=  piece_count * modifier * piece_type.value();
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval_start_position() {
        let board = Board::default();

        assert_eq!(BoardEvaluation::evaluate(&board), 0);
    }
}