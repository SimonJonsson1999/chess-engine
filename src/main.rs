use chess_engine::board::Board;
use chess_engine::board::MoveGenerator;
use chess_engine::piece::{MoveKind, PieceMove};
use chess_engine::square::Square;
fn main() {
    let mut board = Board::default();
    board.move_piece(PieceMove::new(
        Square::H2,
        Square::H4,
        MoveKind::DoublePawnPush,
    ));
    board.move_piece(PieceMove::new(
        Square::G7,
        Square::G5,
        MoveKind::DoublePawnPush,
    ));
    board.move_piece(PieceMove::new(
        Square::H4,
        Square::G5,
        MoveKind::Capture,
    ));
    board.move_piece(PieceMove::new(
        Square::A7,
        Square::A5,
        MoveKind::DoublePawnPush,
    ));
    board.move_piece(PieceMove::new(
        Square::G5,
        Square::G6,
        MoveKind::Quiet,
    ));
    board.move_piece(PieceMove::new(
        Square::H7,
        Square::H6,
        MoveKind::Quiet,
    ));
    board.move_piece(PieceMove::new(
        Square::H1,
        Square::H5,
        MoveKind::Quiet,
    ));
    MoveGenerator::generate_all_moves(&board, chess_engine::piece::Color::White);
    board.show();
}
