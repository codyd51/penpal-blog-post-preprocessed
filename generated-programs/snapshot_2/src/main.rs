use std::io;
use crate::board::Board;
mod board;

fn main() -> io::Result<()> {
    println!("Game started!");
    let board = Board::new();
    println!("{board}");

    Ok(())
}
