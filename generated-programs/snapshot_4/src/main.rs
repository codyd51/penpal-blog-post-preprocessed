use std::io;
use crate::board::Board;
mod board;

fn main() -> io::Result<()> {
    println!("Game started!");
    let mut board = Board::new();

    // Spawn a couple tiles
    board.spawn_tile_in_random_location();
    board.spawn_tile_in_random_location();

    // Show the initial state of the board
    println!("{board}");


    Ok(())
}
