use std::fmt;
use std::ops::{Add, Index, IndexMut, Sub};

// This will define squares as A1=0, B1=1, ...., H8=63
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    pub const fn from_index(index: u8) -> Self {
        debug_assert!(index < 64);
        unsafe { std::mem::transmute(index) }
    }

    pub const fn index(self) -> u8 {
        self as u8
    }

    pub const fn rank(self) -> u8 {
        self.index() / 8
    }
    pub const fn file(self) -> u8 {
        self.index() % 8
    }
}

impl Add<u8> for Square {
    type Output = Square;

    fn add(self, rhs: u8) -> Self::Output {
        let index = self.index() + rhs;
        assert!(index < 64);

        Square::from_index(index)
    }
}

impl Sub<u8> for Square {
    type Output = Square;

    fn sub(self, rhs: u8) -> Self::Output {
        let index = self.index().checked_sub(rhs).expect("Square underflow");

        Square::from_index(index)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let index = self.index();

        let file = (b'a' + (index % 8)) as char;
        let rank = (index / 8) + 1;

        write!(f, "{}{}", file, rank)
    }
}

#[derive(Clone)]
pub struct SquareMap<T>([T; 64]);

impl<T: Copy> SquareMap<T> {
    pub fn new(data: [T; 64]) -> Self {
        Self(data)
    }
    pub fn filled(value: T) -> Self {
        Self([value; 64])
    }
}

impl<T> Index<Square> for SquareMap<T> {
    type Output = T;

    fn index(&self, square: Square) -> &Self::Output {
        &self.0[square as usize]
    }
}

impl<T> IndexMut<Square> for SquareMap<T> {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self.0[square as usize]
    }
}
