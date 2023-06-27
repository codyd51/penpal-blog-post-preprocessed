use wasm_bindgen::prelude::*;
mod board;
use crate::board::{Board, CellContents};

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn game_start() -> String {
    let mut out = String::new();
    out.push_str("Game started!\n");
    let mut board = Board::new();
    // Spawn a couple tiles
    board.spawn_tile_in_random_location();
    board.spawn_tile_in_random_location();

    // Show the initial state of the board
    out.push_str(&format!("{board}\n"));

    out
}
