use chess_engine::engine::Engine;
use chess_engine::evaluation::BoardEvaluation;
use chess_engine::player::{PlayerType, Computer};
use chess_engine::search::NegaMaxAlphaBetaSearcher;
use chess_engine::ui::ChessUI;

fn main() {
    let evaluator = BoardEvaluation{};
    let searcher = NegaMaxAlphaBetaSearcher::new(evaluator);
    let white = PlayerType::Human;
    let black = PlayerType::AI(Box::new(
        Computer::new(
            searcher,
            4,
        ),
    ));

    let engine = Engine::new(white, black);

    let mut chess_game = ChessUI::new(engine);

    if let Err(e) = chess_game.run() {
        eprintln!("Error running UI: {e}");
    }
}