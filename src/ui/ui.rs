use sdl3::EventPump;
use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;
use sdl3::pixels::{Color};
use sdl3::rect::Rect;
use sdl3::render::{Canvas};
use sdl3::Error;
use sdl3::rect::Point;
use sdl3::mouse::MouseButton;
use sdl3::video::Window;
use std::time::Duration;

use crate::bitboard::BitBoard;
use crate::engine::Engine;
use crate::move_gen::BOARDWIDTH;
use crate::ui::assets::Assets;
use crate::square::Square;
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


// Draw all legal moves from selected piece

// Simplify the highlighted square and selected piece. Could probably use the selected
// piece for changing the colkor of the square

pub struct ChessUI {
    engine: Engine,
    // window_size.0 is width, window_size.1 is height
    window_size: (u32, u32),
    highlighted_squares: BitBoard,
    selected_square: Option<Square>,

}

impl ChessUI {
    pub fn new() -> Self {
        ChessUI {
            engine: Engine::new(),
            window_size: (800, 800),
            highlighted_squares: BitBoard(0),
            selected_square: None
        }
    }

    pub fn run(&mut self) -> Result<(), Error>{
        let (mut canvas, mut events) = self.init();
        let texture_creator = canvas.texture_creator();
        let images = Assets::new(&texture_creator)?;
        
        'running: loop {
            for event in events.poll_iter() {
                // match dbg!(event) {
                match event {
                    // Handle mouseclicks
                    Event::MouseButtonDown {mouse_btn, x, y , ..} => {
                        if mouse_btn == MouseButton::Left {
                            self.highlighted_squares = BitBoard(0);
                            let mouse_pos = Point::new(x as i32, y as i32);
                            let board_rect = self.get_board_rect();
                            if board_rect.contains_point(mouse_pos) {
                                // handle board input
                               
                                let square = self.get_clicked_square(mouse_pos, &board_rect);
                                self.highlighted_squares.set(square);
                                match self.selected_square {
                                    Some(from) => {
                                        self.engine.make_move(from, square);
                                        self.selected_square = None;
                                    },
                                
                                    None => self.selected_square = Some(square)
                                }

                            }
                            else {
                                // Handle UI input

                            }
                        }
                    }
                    // Handle quitting
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
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
            }
            canvas.set_draw_color(Color::RGB(30, 30, 30));
            canvas.clear();

            // Draw everything
            self.draw_board(&mut canvas, &images)?;
            // draw_pieces(&mut canvas);
            canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }



    pub fn get_board_rect(&self) -> Rect {
        let side = min(self.window_size.0, self.window_size.1);

        Rect::new(0, 0, side, side)
    }


    pub fn get_clicked_square(&self, point: Point, board_rect: &Rect) -> Square {
        let local_x = point.x - board_rect.x;
        let local_y = point.y - board_rect.y;
        let tile_size = self.tile_size() as i32;
        let file = local_x / tile_size;
        let screen_rank = local_y / tile_size;
        let board_rank = BOARDWIDTH as i32 - 1 - screen_rank;

        Square::from_rank_file(board_rank as u8, file as u8)
    }

    fn tile_size(&self) -> u32 {
        let min_side = min(self.window_size.0, self.window_size.1);
        min_side / BOARDWIDTH as u32
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
    fn highlighted_square(&self, square: Square) -> bool{
        self.highlighted_squares.is_set(square)
    }
    pub fn draw_board(&mut self, canvas: &mut Canvas<Window>, images: &Assets)  -> Result<(), Error> 
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
                if (rank + file) % 2 == 0 {
                    if self.highlighted_square(piece_square) {
                        canvas.set_draw_color(Color::RGB(246, 246, 105)); // highlighted light
                    }
                    else {
                        canvas.set_draw_color(Color::RGB(240, 217, 181)); // light
                    }
                    
                } else {
                    if self.highlighted_square(piece_square) {
                        canvas.set_draw_color(Color::RGB(246, 246, 105)); // highlighted light
                    }
                    else {
                        canvas.set_draw_color(Color::RGB(181, 136, 99)); // dark
                    }
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
}
