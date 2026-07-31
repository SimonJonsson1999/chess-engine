use sdl3::EventPump;
use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;
use sdl3::pixels::{Color, PixelFormat};
use sdl3::rect::Rect;
use sdl3::render::{Canvas};
use sdl3::Error;
use sdl3::surface::Surface;
use sdl3::video::Window;
use std::time::Duration;

use crate::engine::Engine;
use crate::move_gen::BOARDWIDTH;
use std::cmp::min;

// TODO

// -- Set icon
// pub fn set_icon<S: AsRef<SurfaceRef>>(&mut self, icon: S) -> bool
// Use this function to set the icon for a window.

// Example:
// ⓘ
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

pub struct ChessUI {
    engine: Engine,
    // window_size.0 is width, window_size.1 is height
    window_size: (u32, u32),
}

impl ChessUI {
    pub fn new() -> Self {
        ChessUI {
            engine: Engine::new(),
            window_size: (800, 800),
        }
    }

    pub fn run(&mut self) -> Result<(), Error>{
        let (mut canvas, mut events) = self.init();
        'running: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }

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
            }
            canvas.set_draw_color(Color::RGB(30, 30, 30));
            canvas.clear();

            // Draw everything
            self.draw_board(&mut canvas)?;
            // draw_pieces(&mut canvas);
            canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }

    pub fn get_board_rect(&self) -> Rect {
        Rect::new(0, 0, self.window_size.0, self.window_size.1)
    }
    pub fn init(&self) -> (Canvas<Window>, EventPump) {
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

    pub fn draw_board(&mut self, canvas: &mut Canvas<Window>)  -> Result<(), Error> 
   {
        let min_side = min(self.window_size.0, self.window_size.1);
        let tile_size = min_side / 8;
        let board_rect = self.get_board_rect();
        for rank in 0..BOARDWIDTH {
            for file in 0..BOARDWIDTH {
                let x = board_rect.x() + file as i32 * tile_size as i32;
                let y = board_rect.y() + rank as i32 * tile_size as i32;

                let square = Rect::new(x, y, tile_size as u32, tile_size as u32);

                if (rank + file) % 2 == 0 {
                    canvas.set_draw_color(Color::RGB(240, 217, 181)); // light
                } else {
                    canvas.set_draw_color(Color::RGB(181, 136, 99)); // dark
                }

                canvas.fill_rect(square)?;
            }
        }
        Ok(())
    }
}
