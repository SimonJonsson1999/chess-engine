use std::io;

use chess_engine::debug::tui::Tui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let result = Tui::new().run(&mut terminal);

    ratatui::restore();

    result
}
