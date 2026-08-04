use sdl3::EventPump;

use sdl3::render::{Canvas};
use sdl3::Error;
use sdl3::video::Window;
use std::println;
use std::time::Duration;
use crate::bitboard::BitBoard;
use crate::engine::Engine;
use crate::ui::assets::Assets;
use crate::square::Square;
use crate::board::GameState;




// TODO

// -- Set icon
// pub fn set_icon<S: AsRef<SurfaceRef>>(&mut self, icon: S) -> bool
// Use this function to set the icon for a window.

// Example:
// // requires "--features 'image'"
// use sdl3::surface::Surface;

// let window_icon = Surface::from_file("/path/to/icon.png")?;
// window.set_icon(window_icon);

// -- Use Viewports
// A viewport is a rectangular region of the canvas with its own local coordinate system.
// Set a viewport, draw using coordinates relative to it, then switch to another viewport.
// Drawing is clipped to the viewport. Only one viewport is active at a time.


// -- Use textures, for example the board
// Board rect (size)
//         ↓
// Create surface of that size
//         ↓
// Draw the chessboard onto the surface
//         ↓
// Create a texture from the surface
//         ↓
// Each frame: render the texture into board_rect


// Draw all legal moves from selected piece

// Simplify the highlighted square and selected piece. Could probably use the selected
// piece for changing the colkor of the square


// Implement drag and drop of pieces instead of just clicking

pub struct ChessUI {
    pub(crate) engine: Engine,
    // window_size.0 is width, window_size.1 is height
    pub(crate) window_size: (u32, u32),
    pub(crate) selected_square: Option<Square>,
    pub(crate) legal_destinations: BitBoard,

}

impl ChessUI {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: engine,
            window_size: (800, 800),
            selected_square: None,
            legal_destinations: BitBoard(0),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let (mut canvas, mut events) = self.init_sdl();
        let texture_creator = canvas.texture_creator();
        let images = Assets::new(&texture_creator)?;

        'running: loop {
            // Handle all pending events
            for event in events.poll_iter() {
                if !self.handle_event(event) {
                    break 'running;
                }
            }
            self.engine.update();
            if self.engine.is_game_over() {
                match self.engine.game_state {
                    GameState::Checkmate => {
                        let winner = self.engine.turn().opposite();

                        println!("Game over! {:?} wins!", winner);
                    }
                    GameState::Stalemate => {
                        println!("Game over! Draw.");
                    }
                    _ => {}
                }
                break 'running;
            }
            
            self.draw(&mut canvas, &images)?;
            std::thread::sleep(Duration::from_millis(16));
        }
        Ok(())
    }

    pub fn init_sdl(&self) -> (Canvas<Window>, EventPump) {
        // Handle stuff where we need the SDL context
        // If i will need sdl outside of this function, might remove later

        let sdl = sdl3::init().expect("Failed to initilize SDL3");
        let canvas = {
            let video = sdl.video().expect("Failed to get display");
            let window = video
                .window("Chess Engine UI", self.window_size.0, self.window_size.1)
                .position_centered()
                .resizable()
                .build()
                .expect("Failed to crate window");
            window.into_canvas()
        };

        let events = sdl.event_pump().expect("Failed to get event loop");

        (canvas, events)
    }

}
