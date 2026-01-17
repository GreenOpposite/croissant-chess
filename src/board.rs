use crate::bitboard::Bitboard;
use crate::castling_rights::{CastlingRight, CastlingRights};
use crate::color::Color;
use crate::color::Color::{Black, White};
use crate::piece::Piece;
use crate::square::Square;
use colored::Colorize;
use std::fmt::{Display, Formatter};

pub const BOARD_SIZE: u8 = 8;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Board {
    current_color: Color,
    pieces: [Piece; 64],
    piece_bitboards: [Bitboard; 12],
    color_bitboards: [Bitboard; 2],
    en_passant_square: Square,
    castling_rights: CastlingRights,
    half_moves: usize,
    full_moves: usize,
}

impl Board {
    pub fn from_fen(fen: &str) -> Result<Self, ()> {
        let mut parts = fen.split(" ");
        let fen_piece_placement = parts.next().unwrap();
        let fen_active_color = parts.next().unwrap();
        let fen_castling_rights = parts.next().unwrap();
        let fen_en_passant = parts.next().unwrap();
        let fen_halfmoves = parts.next().unwrap_or("0");
        let fen_fullmoves = parts.next().unwrap_or("1");

        let mut board = Board::default();

        let mut index = 0u8;
        for char in fen_piece_placement.chars() {
            // FEN starts at the top left (a8). My internal representation has a1 as 0 => We have to
            // invert the rank with 7 - rank to start adding piece at the board from the bottom.
            let rank = index / BOARD_SIZE;
            let file = index % BOARD_SIZE;
            let square = Square::from_file_and_rank(file, 7 - rank);
            match char {
                '/' => {
                    index -= 1; // Index is always incremented after match -> decrement here to leave it at the same value
                }
                num if num.is_ascii_digit() => {
                    let num = char::to_digit(num, 10).unwrap() as u8;
                    index += num - 1;
                }
                x => {
                    if let Some(piece) = Piece::from_char(x) {
                        board.add_piece(piece, square);
                    } else {
                        return Err(());
                    }
                }
            }
            index += 1;
        }

        board.current_color = match fen_active_color {
            "w" => White,
            "b" => Black,
            _ => return Err(()),
        };

        for char in fen_castling_rights.chars() {
            match char {
                'K' => board.castling_rights.add(CastlingRight::WhiteKingside),
                'k' => board.castling_rights.add(CastlingRight::BlackKingside),
                'Q' => board.castling_rights.add(CastlingRight::WhiteQueenside),
                'q' => board.castling_rights.add(CastlingRight::BlackQueenside),
                '-' => {}
                x => println!("Unexpected char {x} in castling rights"),
            }
        }

        board.half_moves = fen_halfmoves.parse().unwrap_or_default();
        board.full_moves = fen_fullmoves.parse().unwrap_or_default();
        board.en_passant_square = Square::from_algebraic_notation(fen_en_passant);

        Ok(board)
    }

    pub fn fen(&self) -> String {
        let mut fen = String::new();

        for rank in (0..8).rev() {
            let mut empty = 0;

            for file in 0..8 {
                let square = Square::from_file_and_rank(file, rank);

                let piece_char = self.pieces[square as usize].char();

                match piece_char {
                    Some(c) => {
                        if empty > 0 {
                            fen.push_str(&empty.to_string());
                            empty = 0;
                        }
                        fen.push(c);
                    }
                    None => empty += 1,
                }
            }

            if empty > 0 {
                fen.push_str(&empty.to_string());
            }

            if rank > 0 {
                fen.push('/');
            }
        }

        // Active color
        fen.push(' ');
        fen.push(self.current_color.char());

        // Castling rights
        fen.push(' ');
        fen.push_str(self.castling_rights.to_string().as_str());

        // En passant
        fen.push(' ');
        if self.en_passant_square != Square::None {
            fen.push_str(format!("{}", self.en_passant_square).as_str());
        } else {
            fen.push('-');
        }

        fen.push(' ');
        fen.push_str(&self.half_moves.to_string());
        fen.push(' ');
        fen.push_str(&self.full_moves.to_string());

        fen
    }

    pub fn add_piece(&mut self, piece: Piece, square: Square) {
        self.piece_bitboards[piece as usize].add(square);
        self.color_bitboards[piece.get_color() as usize].add(square);
        self.pieces[square as usize] = piece;
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            current_color: Color::default(),
            pieces: [Piece::default(); 64],
            piece_bitboards: [Bitboard::default(); 12],
            color_bitboards: [Bitboard::default(); 2],
            en_passant_square: Default::default(),
            castling_rights: Default::default(),
            half_moves: 0,
            full_moves: 1,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "   {}", self.fen())?;

        for rank in (0..BOARD_SIZE).rev() {
            write!(f, " {} ", rank + 1)?;

            for file in 0..BOARD_SIZE {
                let square = Square::from_file_and_rank(file, rank);
                let piece = self.pieces[square as usize];
                let cell = match piece {
                    Piece::None => "   ".to_owned(),
                    _ => format!(" {} ", piece),
                };
                let dark = (rank + file) % 2 == 0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_fen_basic() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert!(board.is_ok());
        let board = board.unwrap();

        assert_eq!(board.color_bitboards[White as usize], Bitboard(65535));
        assert_eq!(
            board.color_bitboards[Black as usize],
            Bitboard(18446462598732840960)
        );

        assert_eq!(board.half_moves, 0);
        assert_eq!(board.full_moves, 1);

        assert_eq!(board.en_passant_square, Square::None);

        assert_eq!(board.current_color, White);

        assert_eq!(board.castling_rights, CastlingRights::all());
    }
}
