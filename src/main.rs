use chess_engine::board::Board;
use chess_engine::board::MoveGenerator;
use chess_engine::piece::{MoveKind, PieceMove};
use chess_engine::square::Square;
fn main() {
    let turn = chess_engine::piece::Color::White;
    let mut board = Board::default();

    let possible_moves = MoveGenerator::generate_all_moves(&board, turn);
    // board.move_piece(random(possible_moves));
    board.show();

    board.move_piece(PieceMove::new(
        Square::E2,
        Square::E4,
        MoveKind::DoublePawnPush,
    ));
    board.show();
    MoveGenerator::generate_all_moves(&board, chess_engine::piece::Color::White);
    board.move_piece(PieceMove::new(
        Square::E7,
        Square::E5,
        MoveKind::DoublePawnPush,
    ));
    board.show();
    MoveGenerator::generate_all_moves(&board, chess_engine::piece::Color::White);
    board.undo();
    board.undo();
    board.show();
}
