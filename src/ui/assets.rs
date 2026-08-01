use std::ops::{Index, IndexMut};
use sdl3::image::{LoadTexture};
use sdl3::render::{Texture, TextureCreator};
use sdl3::video::WindowContext;
use crate::piece::{Piece};
use sdl3::Error;
pub struct Assets<'a> {
    pub piece_textures: [[Texture<'a>; 6]; 2],
}
impl<'a> Assets<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, Error> {
        Ok(Self {
            piece_textures: [
                [
                    texture_creator.load_texture("assets/pieces/wP.png")?,
                    texture_creator.load_texture("assets/pieces/wN.png")?,
                    texture_creator.load_texture("assets/pieces/wB.png")?,
                    texture_creator.load_texture("assets/pieces/wR.png")?,
                    texture_creator.load_texture("assets/pieces/wQ.png")?,
                    texture_creator.load_texture("assets/pieces/wK.png")?,
                ],
                [
                    texture_creator.load_texture("assets/pieces/bP.png")?,
                    texture_creator.load_texture("assets/pieces/bN.png")?,
                    texture_creator.load_texture("assets/pieces/bB.png")?,
                    texture_creator.load_texture("assets/pieces/bR.png")?,
                    texture_creator.load_texture("assets/pieces/bQ.png")?,
                    texture_creator.load_texture("assets/pieces/bK.png")?,
                ],
            ],
        })
    }
}
impl<'a> Index<Piece> for Assets<'a> {
    type Output = Texture<'a>;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self.piece_textures[piece.color as usize][piece.piece_type as usize]
    }
}

impl<'a> IndexMut<Piece> for Assets<'a> {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self.piece_textures[piece.color as usize][piece.piece_type as usize]
    }
}