use chess_engine::square::Square;
use chess_engine::game::Game;
fn main() {
    let mut game = Game::new();

    game.make_move(Square::E2, Square::E4);
    game.make_move(Square::D7, Square::D5);
    game.make_move(Square::H2, Square::H4);
    game.make_move(Square::G7, Square::G5);
    game.make_move(Square::H4, Square::G5);
    game.make_move(Square::A7, Square::A5);
    game.make_move(Square::G5, Square::G6);
    game.make_move(Square::H7, Square::H6);
    game.make_move(Square::H1, Square::H5);

    game.show();
}
