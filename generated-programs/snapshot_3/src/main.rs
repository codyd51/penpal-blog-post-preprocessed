use std::io;
use crate::board::{Board, CellContents, BOARD_WIDTH, BOARD_HEIGHT};
mod board;

fn main() -> io::Result<()> {
    println!("Game started!");
    let mut board = Board::new();
    for i in 0..16 {
        board.cells[i].contents = CellContents::Occupied(i)
    }
    println!("{board}");

    Ok(())
}
