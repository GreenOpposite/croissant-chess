use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    pub fn new(value: u8) -> Self {
        debug_assert!(value < 2);
        unsafe { std::mem::transmute(value) }
    }
    pub fn other(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn char(self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}

impl<T> Index<Color> for [T] {
    type Output = T;

    fn index(&self, color: Color) -> &Self::Output {
        self.get(color as usize).unwrap()
    }
}

impl<T> IndexMut<Color> for [T] {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        self.get_mut(color as usize).unwrap()
    }
}
