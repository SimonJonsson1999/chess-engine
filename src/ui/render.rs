use sdl3::render::{Canvas};
use sdl3::video::Window;
use sdl3::pixels::{Color};
use sdl3::Error;
use sdl3::rect::{Rect, Point};
use crate::ui::assets::Assets;
use crate::ui::ChessUI;
use crate::square::Square;
use crate::move_gen::BOARDWIDTH;

impl ChessUI {
    pub(crate) fn draw(&mut self, canvas: &mut Canvas<Window>, images: &Assets) -> Result<(), Error> {
            // Draw one frame
            canvas.set_draw_color(Color::RGB(30, 30, 30));
            canvas.clear();

            self.draw_board(canvas, &images)?;

            canvas.present();
            Ok(())
    }

    pub(crate) fn draw_board(&mut self, canvas: &mut Canvas<Window>, images: &Assets)  -> Result<(), Error> 
    {
        let tile_size = self.tile_size();
        let board_rect = self.get_board_rect();
        for rank in 0..BOARDWIDTH {
            for file in 0..BOARDWIDTH {
                let x = board_rect.x() + file as i32 * tile_size as i32;
                let y = board_rect.y() + rank as i32 * tile_size as i32;

                let square = Rect::new(x, y, tile_size as u32, tile_size as u32);
                let board_rank = BOARDWIDTH - 1 - rank;
                let piece_square = Square::from_rank_file(board_rank as u8, file as u8);
                
                if self.selected_square(piece_square) {
                        canvas.set_draw_color(Color::RGB(246, 246, 105)); // highlighted light
                }
                else if self.legal_destinations.is_set(piece_square) {
                    self.draw_move_circle(canvas, square)?;
                }
                else if (rank + file) % 2 == 0 {
                    canvas.set_draw_color(Color::RGB(240, 217, 181)) // light
                    
                } else {
                        canvas.set_draw_color(Color::RGB(181, 136, 99)); // dark
                }

                canvas.fill_rect(square)?;
                // Create API to get piece, not by getting the inner structure like this
                if let Some(piece) = self.engine.board.piece_on_square[piece_square] {
                    canvas.copy(
                        &images[piece],
                        None,
                        square,
                    )?;
                }
            }
                    
        }
        Ok(())
    }

    pub(crate) fn selected_square(&self, square: Square) -> bool {
        match self.selected_square {
            Some(selected_square) => {
                square == selected_square
            },
            None => false
        }
    }
    pub(crate) fn draw_move_circle(
        &self,
        canvas: &mut Canvas<Window>,
        square: Rect,
    ) -> Result<(), Error> {
        let cx = square.x() + square.width() as i32 / 2;
        let cy = square.y() + square.height() as i32 / 2;

        let radius = square.width().min(square.height()) as i32 / 6;

        canvas.set_draw_color(Color::RGBA(80, 80, 80, 255));

        for dy in -radius..=radius {
            let dx = ((radius * radius - dy * dy) as f32).sqrt() as i32;

            canvas.draw_line(
                Point::new(cx - dx, cy + dy),
                Point::new(cx + dx, cy + dy),
            )?;
        }

        Ok(())
    }
}