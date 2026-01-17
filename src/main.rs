use crate::board::Board;

mod bitboard;
mod board;
mod castling_rights;
mod color;
mod moves;
mod piece;
mod square;

fn main() {
    let board = Board::starting_position();

    println!("{board}")
}
