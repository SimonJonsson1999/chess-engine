use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;
use sdl3::rect::{Rect, Point};
use sdl3::mouse::MouseButton;

use crate::ui::ChessUI;
use crate::bitboard::BitBoard;
use crate::move_gen::BOARDWIDTH;
use crate::square::Square;

impl ChessUI {
    pub(crate) fn handle_event(&mut self, event: Event) -> bool {
        // match dbg!(event) {
        match event {
                    // Handle mouseclicks
            Event::MouseButtonDown {mouse_btn, x, y , ..} => {
                if mouse_btn == MouseButton::Left {
                    let mouse_pos = Point::new(x as i32, y as i32);
                    let board_rect = self.get_board_rect();

                    if !board_rect.contains_point(mouse_pos) {
                        return true;
                    }
                    self.legal_destinations = BitBoard(0);
                    let square = self.get_clicked_square(mouse_pos, &board_rect);

                    // Did we click one of our own pieces?
                    if let Some(piece) = self.engine.board.piece_on_square[square] {
                        if piece.color == self.engine.board.turn {
                            self.selected_square = Some(square);
                            self.set_legal_destinations(square);
                            return true;
                        }
                    }

                    // Otherwise, try to move the currently selected piece.
                    if let Some(from) = self.selected_square {
                        if self.engine.make_move(from, square) {
                            self.selected_square = None;
                            self.legal_destinations = BitBoard(0);
                        }
                    }
                }
            }
        // Handle quitting
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => {
            return false;
        }
        // Handle windowsize updates
        Event::Window {
            win_event: WindowEvent::Resized(width, height),
            ..
        } => {
            println!("Window resized to {}x{}", width, height);
            // Recalculate layout here
            self.window_size = (width as u32, height as u32);
        }

        _ => {}
            }
            return true
    }

    pub(crate) fn get_clicked_square(&self, point: Point, board_rect: &Rect) -> Square {
        let local_x = point.x - board_rect.x;
        let local_y = point.y - board_rect.y;
        let tile_size = self.tile_size() as i32;
        let file = local_x / tile_size;
        let screen_rank = local_y / tile_size;
        let board_rank = BOARDWIDTH as i32 - 1 - screen_rank;
        Square::from_rank_file(board_rank as u8, file as u8)
    }

    pub(crate) fn set_legal_destinations(&mut self, square: Square) {
        for square in self.engine.attacked_squares(square) {
            self.legal_destinations.set(square);
        }
    }
                    
}