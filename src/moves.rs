use crate::piece::Piece;
use crate::square::Square;
use crate::square::Square::{A1, A8, C1, C8, D1, D8, F1, F8, G1, G8, H1, H8};
use std::fmt::{Display, Formatter};

/// # Memory Layout
/// \[FLAGS: 4 bit]\[TO: 6 bit]\[FROM: 6 bit]
/// Flags are of type [MoveType]
#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct Move(u16);

/// Values taken from [here](https://www.chessprogramming.org/Encoding_Moves#From-To_Based)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MoveType {
    Normal = 0b0000,
    DoublePush = 0b0001,
    Castling = 0b0010,
    Capture = 0b0100,
    EnPassant = 0b0101,
    PromotionKnight = 0b1000,
    PromotionBishop = 0b1001,
    PromotionRook = 0b1010,
    PromotionQueen = 0b1011,
    PromotionCaptureKnight = 0b1100,
    PromotionCaptureBishop = 0b1101,
    PromotionCaptureRook = 0b1110,
    PromotionCaptureQueen = 0b1111,
}

impl Move {
    pub fn new(from: Square, to: Square, ty: MoveType) -> Self {
        Self(from as u16 | ((to as u16) << 6) | ((ty as u16) << 12))
    }

    pub fn from(self) -> Square {
        Square::new((self.0 & 0b111111) as u8)
    }

    pub fn to(self) -> Square {
        Square::new(((self.0 & (0b111111 << 6)) >> 6) as u8)
    }

    pub fn get_rook_from_to_square_castling(&self) -> (Square, Square) {
        match self.to() {
            G1 => (H1, F1),
            C1 => (A1, D1),
            G8 => (H8, F8),
            C8 => (A8, D8),
            _ => (Square::None, Square::None),
        }
    }

    pub fn ty(self) -> MoveType {
        unsafe { std::mem::transmute((self.0 >> 12) as u8) }
    }

    pub fn get_promotion(&self) -> Option<Piece> {
        match self.ty() {
            MoveType::PromotionKnight | MoveType::PromotionCaptureKnight => {
                Some(Piece::WhiteBishop)
            }
            MoveType::PromotionRook | MoveType::PromotionCaptureRook => Some(Piece::WhiteRook),
            MoveType::PromotionBishop | MoveType::PromotionCaptureBishop => {
                Some(Piece::WhiteBishop)
            }
            MoveType::PromotionQueen | MoveType::PromotionCaptureQueen => Some(Piece::WhiteQueen),
            _ => None,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(promotion) = self.get_promotion() {
            writeln!(
                f,
                "{} → {} promoting to {}",
                self.from(),
                self.to(),
                promotion
            )
        } else {
            writeln!(f, "{} → {}", self.from(), self.to())
        }
    }
}
