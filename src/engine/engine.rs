use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::move_gen::MoveGenerator;
use crate::square::Square;
use crate::search::Search;
use crate::piece::Color;
use crate::board::GameState;
pub struct Engine {
    pub board: Board,
    pub game_state: GameState,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::default(),
            game_state: GameState::Ongoing,
        }
    }

    pub fn make_move(&mut self, from: Square, to: Square) -> bool{
        let all_moves = MoveGenerator::generate_valid_moves(&mut self.board);
        let next_move = all_moves.iter().find(|mv| mv.from == from && mv.to == to);
        match next_move {
            Some(next_move) => {
                self.board.move_piece(*next_move);
                self.update_game_state();
                return true
            }
            None => {
                print!("Move from {from} to {to} was not a legal move");
                return false
            }
        }
        
    }
    pub fn show(&self) {
        self.board.show();
    }

    pub fn attacked_squares(&mut self, square: Square) -> Vec<Square>{
        let attacked: BitBoard = MoveGenerator::generate_valid_moves_from(&mut self.board, square)
                                                                    .into_iter()
                                                                    .fold(BitBoard(0), |mut bb, mv| {
                                                                        bb.set(mv.to);
                                                                        bb
                                                                    });
        let moves_from_sq = attacked.squares();
        moves_from_sq
    }
    pub fn turn(&self) -> Color {
        self.board.turn
    }

    pub fn make_best_move(&mut self) {
        let best_move = Search::best_move(&mut self.board, 3);
        self.board.move_piece(best_move);
        self.update_game_state();
    }
    fn update_game_state(&mut self) {
        self.game_state = self.board.game_state();
    }
} // Impl Engine
