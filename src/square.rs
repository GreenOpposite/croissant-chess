use crate::bitboard::Bitboard;
use crate::board::BOARD_SIZE;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

/// Square representation with a1 = 0.
///
/// Same implementation as used by [Reckless][R]
///
/// [R]: [https://github.com/codedeliveryservice/Reckless]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
    #[default]
    None,
}

impl Square {
    pub fn new(square: u8) -> Self {
        debug_assert!(square <= 64);
        unsafe { std::mem::transmute(square) }
    }

    pub fn from_file_and_rank(file: u8, rank: u8) -> Self {
        debug_assert!(file < 8);
        debug_assert!(rank < 8);

        Self::new(file | (rank << 3))
    }

    /// Returns [Square::None] on invalid input
    pub fn from_algebraic_notation(notation: &str) -> Self {
        let b = notation.as_bytes();
        if b.len() != 2 || b[0] < b'a' || b[0] > b'h' || b[1] < b'1' || b[1] > b'8' {
            return Self::None;
        }

        let file = b[0] - b'a';
        let rank = b[1] - b'1';
        Self::from_file_and_rank(file, rank)
    }

    pub fn bitboard(self) -> Bitboard {
        Bitboard(1 << self as u8)
    }

    pub fn file(&self) -> u8 {
        *self as u8 % BOARD_SIZE
    }

    pub fn rank(&self) -> u8 {
        *self as u8 / BOARD_SIZE
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if *self as u8 > 64 {
            return Err(std::fmt::Error);
        }

        let (file, rank) = (self.file(), self.rank());

        let file_char = (file + b'a') as char;
        let rank_char = rank + 1;

        write!(f, "{}{}", file_char, rank_char)
    }
}

impl<T> Index<Square> for [T] {
    type Output = T;

    fn index(&self, square: Square) -> &Self::Output {
        self.get(square as usize).unwrap()
    }
}

impl<T> IndexMut<Square> for [T] {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        self.get_mut(square as usize).unwrap()
    }
}
