use crate::board::Board;

mod bitboard;
mod board;
mod castling_rights;
mod color;
mod piece;
mod square;

fn main() {
    let board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    println!("{board}")
}
