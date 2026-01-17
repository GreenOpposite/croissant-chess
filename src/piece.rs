use crate::color::Color;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

/// Same implementation as used by [Reckless][R]
///
/// [R]: [https://github.com/codedeliveryservice/Reckless]
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
    #[default]
    None,
}

impl Piece {
    pub fn new(value: u8) -> Self {
        debug_assert!(value < 13);
        unsafe { std::mem::transmute(value) }
    }

    pub fn with_color(self, color: Color) -> Self {
        Self::new(match color {
            Color::White => (self as u8) & 0b11111110,
            Color::Black => (self as u8) | 1,
        })
    }

    pub fn get_color(&self) -> Color {
        // Color is encoded in the lsb (0 => White, 1 => Black)
        let color = *self as usize & 1;
        Color::new(color as u8)
    }

    pub fn from_char(c: char) -> Option<Piece> {
        Some(Self::new("PpNnBbRrQqKk".find(c)? as u8))
    }

    pub fn char(&self) -> Option<char> {
        match self {
            Piece::WhitePawn => Some('P'),
            Piece::BlackPawn => Some('p'),
            Piece::WhiteKnight => Some('N'),
            Piece::BlackKnight => Some('n'),
            Piece::WhiteBishop => Some('B'),
            Piece::BlackBishop => Some('b'),
            Piece::WhiteRook => Some('R'),
            Piece::BlackRook => Some('r'),
            Piece::WhiteQueen => Some('Q'),
            Piece::BlackQueen => Some('q'),
            Piece::WhiteKing => Some('K'),
            Piece::BlackKing => Some('k'),
            Piece::None => None,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Piece::WhitePawn => '♙',
            Piece::BlackPawn => '♟',
            Piece::WhiteKnight => '♘',
            Piece::BlackKnight => '♞',
            Piece::WhiteBishop => '♗',
            Piece::BlackBishop => '♝',
            Piece::WhiteRook => '♖',
            Piece::BlackRook => '♜',
            Piece::WhiteQueen => '♕',
            Piece::BlackQueen => '♛',
            Piece::WhiteKing => '♔',
            Piece::BlackKing => '♚',
            Piece::None => ' ',
        };

        write!(f, "{}", char)
    }
}

impl<T> Index<Piece> for [T] {
    type Output = T;

    fn index(&self, piece: Piece) -> &Self::Output {
        self.get(piece as usize).unwrap()
    }
}

impl<T> IndexMut<Piece> for [T] {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        self.get_mut(piece as usize).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_char() {
        assert_eq!(Piece::from_char('P'), Some(Piece::WhitePawn));
        assert_eq!(Piece::from_char('p'), Some(Piece::BlackPawn));

        assert_eq!(Piece::from_char('N'), Some(Piece::WhiteKnight));
        assert_eq!(Piece::from_char('n'), Some(Piece::BlackKnight));

        assert_eq!(Piece::from_char('B'), Some(Piece::WhiteBishop));
        assert_eq!(Piece::from_char('b'), Some(Piece::BlackBishop));

        assert_eq!(Piece::from_char('R'), Some(Piece::WhiteRook));
        assert_eq!(Piece::from_char('r'), Some(Piece::BlackRook));

        assert_eq!(Piece::from_char('Q'), Some(Piece::WhiteQueen));
        assert_eq!(Piece::from_char('q'), Some(Piece::BlackQueen));

        assert_eq!(Piece::from_char('K'), Some(Piece::WhiteKing));
        assert_eq!(Piece::from_char('k'), Some(Piece::BlackKing));

        assert_eq!(Piece::from_char('-'), None);
    }
}
