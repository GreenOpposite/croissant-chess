use crate::board::Board;
use crate::moves::Move;
use crate::moves::MoveType::Castling;
use crate::square::Square::{E8, G8};

mod bitboard;
mod board;
mod castling_rights;
mod color;
mod moves;
mod piece;
mod square;

fn main() {
    let mut board =
        Board::from_fen("r3k2r/pp1p1ppp/8/1Bp1p3/4P3/5N2/PPPP1PPP/R3K2R b KQkq - 0 1").unwrap();
    println!("{board}");

    let mv = Move::new(E8, G8, Castling);
    board.make_move(mv);
    println!("{board}");
}
