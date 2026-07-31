use sdl3::pixels::Color;
use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;
use std::time::Duration;
use sdl3::rect::Rect;
use sdl3::video::{Window};
use sdl3::render::Canvas;

use std::cmp::min;
use crate::engine::Engine;
use crate::move_gen::BOARDWIDTH;
pub struct ChessUI {
    engine: Engine,
    // window_size.0 is width, window_size.1 is height
    window_size: (u32, u32),
}

impl ChessUI {
    pub fn new() -> Self {
        ChessUI{
            engine: Engine::new(),
            window_size: (800, 600)
        }
    }

    pub fn run(&mut self) {
        
        let sdl = sdl3::init().expect("Failed to initilize SDL3");
        
        let mut canvas = {
            let video = sdl.video().expect("Failed to get display");
            let window = video
            .window("Chess Engine UI", self.window_size.0, self.window_size.1)
            .position_centered()
            .resizable()
            .build()
            .expect("Failed to crate window");

            window
            .into_canvas()

        };
        

        let mut events = sdl.event_pump().expect("Failed to get event loop");
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
            self.draw_board(&mut canvas);
            // draw_pieces(&mut canvas);
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn get_board_rect(&self) -> Rect {
        Rect::new(
            0,
            0,
            self.window_size.0,
            self.window_size.1,
        )
    } 

    pub fn draw_board(&mut self, canvas: &mut Canvas<Window>) {
        let min_side = min(self.window_size.0, self.window_size.1);
        let tile_size = min_side / 8;
        let board_rect = self.get_board_rect();
        for rank in 0..BOARDWIDTH {
            for file in 0..BOARDWIDTH {
                let x = board_rect.x() + file as i32 * tile_size as i32;
                let y = board_rect.y() + rank as i32 * tile_size as i32;

                let square = Rect::new(
                    x,
                    y,
                    tile_size as u32,
                    tile_size as u32,
                );

                if (rank + file) % 2 == 0 {
                    canvas.set_draw_color(Color::RGB(240, 217, 181)); // light
                } else {
                    canvas.set_draw_color(Color::RGB(181, 136, 99)); // dark
                }

                canvas.fill_rect(square);
            }
        }
    }
}