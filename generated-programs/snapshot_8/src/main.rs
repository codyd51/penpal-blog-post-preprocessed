use std::io;
use std::io::BufRead;
use crate::board::Board;
use input::Direction;
mod board;
mod input;

fn main() -> io::Result<()> {
    println!("Game started!");
    let mut board = Board::new();

    // Spawn a couple tiles
    board.spawn_tile_in_random_location();
    board.spawn_tile_in_random_location();

    // Show the initial state of the board
    println!("{board}");

    let stdin = io::stdin();
    for maybe_next_line_of_input in stdin.lock().lines() {
        if let Err(e) = maybe_next_line_of_input {
            return Err(e);
        }

        let next_line_of_input = maybe_next_line_of_input.unwrap();
        let direction = match Direction::try_from(next_line_of_input.as_ref()) {
            Ok(d) => d,
            Err(_) => {
                println!("Unrecognized input!");
                continue;
            },
        };
        println!("Processing {direction:?}");
        board.press(direction);
        if board.is_full() {
            println!("Game over!");
            // Reset to an empty board
            board.empty();
            // And spawn an initial tile (the second will be spawned just below)
            board.spawn_tile_in_random_location();
        }

        board.spawn_tile_in_random_location();
        // Show the new state of the board
        println!("{board}");
    }

    Ok(())
}
