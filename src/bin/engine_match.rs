use chess_engine::evaluation::BoardEvaluation;
use chess_engine::player::{AI, Computer};
use chess_engine::search::NegaMaxAlphaBetaSearcher;

#[cfg(not(feature = "benchmark"))]
use chess_engine::engine::Engine;
#[cfg(not(feature = "benchmark"))]
use chess_engine::player::PlayerType;
#[cfg(not(feature = "benchmark"))]
use chess_engine::ui::ChessUI;

#[cfg(feature = "benchmark")]
#[path = "engine_match/benchmark.rs"]
mod benchmark;

#[derive(Clone, Copy)]
struct EngineConfig {
    name: &'static str,
    create: fn() -> Box<dyn AI>,
}

// Edit these factories and their names to compare other searchers or evaluators.
const ENGINE_A: EngineConfig = EngineConfig {
    name: "A: Negamax depth 2",
    create: || {
        Box::new(Computer::new(
            NegaMaxAlphaBetaSearcher::new(BoardEvaluation),
            2,
        ))
    },
};

const ENGINE_B: EngineConfig = EngineConfig {
    name: "B: Negamax depth 5",
    create: || {
        Box::new(Computer::new(
            NegaMaxAlphaBetaSearcher::new(BoardEvaluation),
            5,
        ))
    },
};

#[cfg(not(feature = "benchmark"))]
fn main() {
    println!("White: {} | Black: {}", ENGINE_A.name, ENGINE_B.name);
    let white = PlayerType::AI((ENGINE_A.create)());
    let black = PlayerType::AI((ENGINE_B.create)());

    let engine = Engine::new(white, black);
    let mut chess_game = ChessUI::new(engine);

    if let Err(e) = chess_game.run() {
        eprintln!("Error running UI: {e}");
    }
}

#[cfg(feature = "benchmark")]
fn main() -> std::process::ExitCode {
    match benchmark::run([ENGINE_A, ENGINE_B]) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Benchmark failed: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}
