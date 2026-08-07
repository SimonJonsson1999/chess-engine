use std::println;

use crate::board::Board;
use crate::board::GameState;
use crate::board::bitboard::BitBoard;
use crate::board::piece::Color;
use crate::board::square::Square;
use crate::move_gen::MoveGenerator;
use crate::player::PlayerType;

pub struct Engine {
    pub board: Board,
    pub game_state: GameState,
    pub white_player: PlayerType,
    pub black_player: PlayerType,
}

impl Engine {
    pub fn new(white_player: PlayerType, black_player: PlayerType) -> Self {
        Self {
            white_player,
            black_player,
            ..Self::default()
        }
    }

    pub fn update(&mut self) {
        if self.is_game_over() {
            return;
        }

        let current_player = match self.turn() {
            Color::White => &mut self.white_player,
            Color::Black => &mut self.black_player,
        };

        match current_player {
            PlayerType::Human => {}
            PlayerType::AI(ai) => {
                let mv = ai.choose_move(&mut self.board);

                self.board.move_piece(mv);
                self.update_game_state();
            }
        }
    }

    pub fn make_move(&mut self, from: Square, to: Square) -> bool {
        let all_moves = MoveGenerator::generate_valid_moves(&mut self.board);
        let next_move = all_moves.iter().find(|mv| mv.from == from && mv.to == to);
        match next_move {
            Some(next_move) => {
                self.board.move_piece(*next_move);
                self.update_game_state();
                return true;
            }
            None => {
                print!("Move from {from} to {to} was not a legal move");
                return false;
            }
        }
    }
    pub fn is_game_over(&self) -> bool {
        match self.game_state {
            GameState::Checkmate | GameState::Stalemate | GameState::FiftyMoveRule => true,
            _ => false,
        }
    }

    pub fn show(&self) {
        self.board.show();
    }

    pub fn attacked_squares(&mut self, square: Square) -> Vec<Square> {
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

    fn update_game_state(&mut self) {
        self.game_state = self.board.game_state();
    }
} // Impl Engine

impl Default for Engine {
    fn default() -> Self {
        Self {
            board: Board::default(),
            game_state: GameState::Ongoing,
            white_player: PlayerType::Human,
            black_player: PlayerType::Human,
        }
    }
}
