use chess_engine::board::{Board, Color, GameState, PieceType};
use chess_engine::evaluation::{BoardEvaluation, Evaluator};
use chess_engine::search::{NegaMaxAlphaBetaSearcher, Searcher};
use std::time::Instant;

#[test]
#[ignore = "Manual headless reproduction of engine_match"]
fn play_depth_two_against_depth_five() {
    let mut board = Board::default();
    let mut white = NegaMaxAlphaBetaSearcher::new(BoardEvaluation);
    let mut black = NegaMaxAlphaBetaSearcher::new(BoardEvaluation);
    let start = Instant::now();
    for ply in 0..160 {
        let state = board.game_state();
        if !matches!(state, GameState::Ongoing | GameState::Check) {
            println!("Result: {state:?}, side to move: {:?}", board.turn);
            return;
        }
        let turn = board.turn;
        let mv = match turn {
            Color::White => white.best_move(&mut board, 2),
            Color::Black => black.best_move(&mut board, 5),
        };
        board.move_piece(mv);
        let material: i32 = [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen]
            .into_iter()
            .map(|piece| {
                (board.bitboards[Color::White][piece].count() as i32
                    - board.bitboards[Color::Black][piece].count() as i32) * piece.value()
            })
            .sum();
        println!("{} {turn:?} {mv}, eval={}, material={material}, elapsed={:.1}s", ply + 1, BoardEvaluation.evaluate(&board), start.elapsed().as_secs_f64());
    }
    println!("Reached the move limit");
}
