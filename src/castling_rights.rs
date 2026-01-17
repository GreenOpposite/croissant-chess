use crate::castling_rights::CastlingRight::{
    BlackKingside, BlackQueenside, WhiteKingside, WhiteQueenside,
};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum CastlingRight {
    WhiteKingside,
    BlackKingside,
    WhiteQueenside,
    BlackQueenside,
}

impl CastlingRight {
    pub fn char(&self) -> char {
        match self {
            CastlingRight::WhiteKingside => 'K',
            CastlingRight::BlackKingside => 'k',
            CastlingRight::WhiteQueenside => 'Q',
            CastlingRight::BlackQueenside => 'q',
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct CastlingRights {
    raw: u8,
}

impl CastlingRights {
    pub fn none() -> Self {
        Self { raw: 0 }
    }

    pub fn all() -> Self {
        Self {
            raw: CastlingRight::WhiteKingside as u8
                | CastlingRight::BlackKingside as u8
                | CastlingRight::WhiteQueenside as u8
                | CastlingRight::BlackQueenside as u8,
        }
    }

    pub fn can_castle(&self, castling_right: CastlingRight) -> bool {
        self.raw & castling_right as u8 != 0
    }

    pub fn add(&mut self, castling_right: CastlingRight) {
        self.raw |= castling_right as u8;
    }

    pub fn remove(&mut self, castling_right: CastlingRight) {
        self.raw &= !(castling_right as u8);
    }
}

impl Display for CastlingRights {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if *self == CastlingRights::none() {
            return write!(f, "-");
        }

        for right in [WhiteKingside, BlackKingside, WhiteQueenside, BlackQueenside].iter() {
            if self.can_castle(*right) {
                write!(f, "{}", right.char())?;
            }
        }

        Ok(())
    }
}
