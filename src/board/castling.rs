#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}
impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
    pub const fn none() -> Self {
        Self {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }
    pub const fn index(&self) -> usize {
        (self.white_kingside as usize)
            | ((self.white_queenside as usize) << 1)
            | ((self.black_kingside as usize) << 2)
            | ((self.black_queenside as usize) << 3)
    }
}
