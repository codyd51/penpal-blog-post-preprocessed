use wasm_bindgen::prelude::*;
mod board;
mod input;
use crate::board::{Board, CellContents};
use crate::input::Direction;

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

lazy_static! {
    static ref BOARD: Mutex<Board> = Mutex::new(Board::new());
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn game_start() -> String {
    let mut out = String::new();
    out.push_str("Game started!\n");

    let mut board = BOARD.lock().unwrap();
    // Clear the board
    for i in 0..16 {
        board.cells[i].contents = CellContents::Empty;
    }
    // Spawn a couple tiles
    board.spawn_tile_in_random_location();
    board.spawn_tile_in_random_location();

    // Show the initial state of the board
    out.push_str(&format!("{board}\n"));

    out
}

#[wasm_bindgen]
pub fn handle_input(direction_str: &str) -> String {
    let direction = Direction::try_from(direction_str).unwrap();
    let mut out = String::new();
    out.push_str(&format!("Processing {direction:?}\n"));
    let mut board = BOARD.lock().unwrap();
    board.press(direction);
    if board.is_full() {
        out.push_str(&format!("Game over!\n"));
        // Reset to an empty board
        board.empty();
        board.spawn_tile_in_random_location();
        board.spawn_tile_in_random_location();
    }
    else {
        board.spawn_tile_in_random_location();
    }
    // Show the new state of the board
    out.push_str(&format!("{board}"));
    out
}
