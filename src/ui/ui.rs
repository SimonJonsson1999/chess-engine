use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;
use sdl3::rect::Rect;
use sdl3::video::Window;
use sdl3::render::Canvas;
use crate::engine::Engine;
use crate::move_gen::BOARDWIDTH;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const SIDEBAR_WIDTH: u32 = 300;
const STATUS_HEIGHT: u32 = 80;




pub struct ChessUI {
    engine: Engine,
    board_rect: Rect,
    board_size: u32,
}

impl ChessUI {
    pub fn new() -> Self {
        let board_size: u32 = (WIDTH - SIDEBAR_WIDTH).min(HEIGHT - STATUS_HEIGHT);
        ChessUI{
            engine: Engine::new(),
            board_rect: ChessUI::get_board_rect(board_size),
            board_size
        }
    }

    pub fn run(&mut self) {
        
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chess Engine UI", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas();

        let mut event_pump = sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
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

    pub fn get_board_rect(board_size: u32) -> Rect {
        Rect::new(
            0,
            0,
            board_size,
            board_size,
        )
    } 

    pub fn draw_board(&mut self, canvas: &mut Canvas<Window>) {
        let tile_size = self.board_size / 8;

        for rank in 0..BOARDWIDTH {
            for file in 0..BOARDWIDTH {
                let x = self.board_rect.x() + file as i32 * tile_size as i32;
                let y = self.board_rect.y() + rank as i32 * tile_size as i32;

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