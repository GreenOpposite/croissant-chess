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
