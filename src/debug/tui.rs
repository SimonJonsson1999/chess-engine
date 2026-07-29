use std::io;

use crate::{bb, bitboard::BitBoard};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Borders, Paragraph},
};

pub struct Tui {
    exit: bool,
    bitboard: BitBoard,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            exit: false,
            bitboard: bb!(A8, D7, A6, G6, E1),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                self.handle_key_event(key);
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let text = format!("{}\n\nPress q to quit.", self.bitboard.debug_grid());
        let paragraph =
            Paragraph::new(text).block(Block::default().title("Chess Debug").borders(Borders::ALL));

        frame.render_widget(paragraph, frame.area());
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        if let KeyCode::Char('q') = key.code {
            self.exit = true;
        }
    }
}
