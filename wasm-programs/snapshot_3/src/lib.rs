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
    for i in 0..16 {
        board.cells[i].contents = CellContents::Occupied(i)
    }
    out.push_str(&format!("{board}\n"));
    out
}
