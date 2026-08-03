use std::cmp::min;
use sdl3::rect::Rect;
use crate::ui::ChessUI;
use crate::move_gen::BOARDWIDTH;

impl ChessUI {
    pub(crate) fn get_board_rect(&self) -> Rect {
        let side = min(self.window_size.0, self.window_size.1);

        Rect::new(0, 0, side, side)
    }

    pub(crate) fn tile_size(&self) -> u32 {
        let min_side = min(self.window_size.0, self.window_size.1);
        min_side / BOARDWIDTH as u32
    }
}