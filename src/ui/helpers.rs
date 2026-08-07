use crate::move_gen::BOARDWIDTH;
use crate::ui::ChessUI;
use sdl3::rect::Rect;
use std::cmp::min;

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
