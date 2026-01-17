use crate::board::BOARD_SIZE;
use crate::square::Square;
use colored::Colorize;
use std::fmt::{Display, Formatter};
use std::iter::FusedIterator;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
#[repr(transparent)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn new(bb: u64) -> Self {
        Self(bb)
    }
    pub const fn empty() -> Self {
        Self(0)
    }
    pub fn pop_count(&self) -> i32 {
        self.0.count_ones() as i32
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn has(&self, square: Square) -> bool {
        (*self & square.bitboard()).is_not_empty()
    }

    pub fn has_any(&self, squares: &[Square]) -> bool {
        for &square in squares {
            if self.has(square) {
                return true;
            }
        }
        false
    }

    pub fn add(&mut self, square: Square) {
        *self = self.bitor(square.bitboard());
    }

    pub fn remove(&mut self, square: Square) {
        *self = self.bitand(!square.bitboard());
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "   0x{:x}", self.0)?;

        for rank in (0..BOARD_SIZE).rev() {
            write!(f, " {} ", rank + 1)?;

            for file in 0..BOARD_SIZE {
                let square = Square::from_file_and_rank(file, rank);
                let dark = (rank + file) % 2 == 0;

                let cell = if self.has(square) {
                    " X ".bold().black()
                } else {
                    "   ".normal()
                };

                if dark {
                    write!(f, "{}", cell.on_green())?;
                } else {
                    write!(f, "{}", cell.on_white())?;
                }
            }
            writeln!(f)?;
        }

        write!(f, "   ")?;
        for file in 0..BOARD_SIZE {
            let c = (b'a' + file) as char;
            write!(f, " {} ", c)?;
        }
        writeln!(f)
    }
}

impl Shl<i32> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: i32) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<i32> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: i32) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 & rhs.0
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 = self.0 ^ rhs.0
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitBoardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIter(self)
    }
}

pub struct BitBoardIter(Bitboard);

impl Iterator for BitBoardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.0 == 0 {
            None
        } else {
            let sq = self.0.0.trailing_zeros() as u8;
            self.0.0 &= self.0.0 - 1; // Clear the least significant bit
            Some(Square::new(sq))
        }
    }
}

impl ExactSizeIterator for BitBoardIter {
    fn len(&self) -> usize {
        self.0.0.count_ones() as usize
    }
}

impl FusedIterator for BitBoardIter {}
