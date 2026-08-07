use crate::board::Board;
use crate::board::piece::{Color, MoveKind, PieceMove, PieceMoveList};
use crate::board::square::Square;

pub struct MoveGenerator {}
impl MoveGenerator {
    pub fn generate_valid_moves(board: &mut Board) -> PieceMoveList {
        let color = board.turn;
        let valid_moves: PieceMoveList = MoveGenerator::generate_all_moves(board, color)
            .iter()
            .copied()
            .filter(|mv| MoveGenerator::legal_move(*mv, board, color))
            .collect();
        valid_moves
    }
    pub fn generate_all_moves(board: &Board, color: Color) -> PieceMoveList {
        let mut possible_moves = PieceMoveList::new();
        MoveGenerator::generate_pawn_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_knight_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_king_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_diag_slider_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_ortogonal_slider_moves(&mut possible_moves, board, color);
        MoveGenerator::generate_castling(&mut possible_moves, board, color);
        possible_moves
    }

    pub fn generate_valid_moves_from(board: &mut Board, from: Square) -> PieceMoveList {
        let moves_from_sq = MoveGenerator::generate_valid_moves(board)
            .iter()
            .copied()
            .filter(|mv| mv.from == from)
            .collect();
        moves_from_sq
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
            let legal = !MoveGenerator::is_square_attacked(board, through, opposite_color)
                && !MoveGenerator::is_square_attacked(board, destination, opposite_color)
                && !MoveGenerator::is_square_attacked(board, board.king(color), opposite_color);
            return legal;
        }
        if psuedo_legal_move.kind == MoveKind::QueenCastle {
            let (through, destination) = match color {
                Color::White => (Square::D1, Square::C1),
                Color::Black => (Square::D8, Square::C8),
            };
            let legal = !MoveGenerator::is_square_attacked(board, through, opposite_color)
                && !MoveGenerator::is_square_attacked(board, destination, opposite_color)
                && !MoveGenerator::is_square_attacked(board, board.king(color), opposite_color);
            return legal;
        };
        board.move_piece(psuedo_legal_move);
        let legal = !MoveGenerator::is_square_attacked(board, board.king(color), opposite_color);
        board.undo();
        return legal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn perft(board: &mut Board, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }

        let moves = MoveGenerator::generate_valid_moves(board);

        let mut nodes = 0;

        for mv in moves.iter() {
            println!("{}", mv);
            board.move_piece(*mv);
            nodes += perft(board, depth - 1);
            board.undo();
        }

        nodes
    }
    #[test]
    fn perft_start_position() {
        let cases = [
            (0, 1),
            (1, 20),
            (2, 400),
            (3, 8_902),
            (4, 197_281),
            (5, 4_865_609),
            // (6, 119_060_324),
            // (7,	3_195_901_860),
        ];

        for (depth, expected) in cases {
            let mut board = Board::default();
            assert_eq!(
                perft(&mut board, depth),
                expected,
                "failed at depth {}",
                depth
            );
        }
    }

    #[test]
    fn perft_kiwipete() {
        let mut board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");

        assert_eq!(perft(&mut board, 1), 48);
        assert_eq!(perft(&mut board, 2), 2_039);
        assert_eq!(perft(&mut board, 3), 97_862);
        assert_eq!(perft(&mut board, 4), 4_085_603);
    }

    #[test]
    fn perft_en_passant() {
        // Test enpassant position
        let mut board = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1");

        assert_eq!(perft(&mut board, 6), 1_440_467);
    }
    #[test]
    fn make_undo_restores_board() {
        let mut board = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1");

        let original = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1");
        for mv in MoveGenerator::generate_valid_moves(&mut board).iter() {
            board.move_piece(*mv);
            board.undo();
            assert_eq!(board, original, "Board differs after {}", *mv);
        }
    }

    #[test]
    fn perft_promotion_heavy() {
        // Test position wiht many promotions
        let mut board =
            Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");

        assert_eq!(perft(&mut board, 1), 6);
        assert_eq!(perft(&mut board, 2), 264);
        assert_eq!(perft(&mut board, 3), 9467);
        assert_eq!(perft(&mut board, 4), 422333);
    }
}
