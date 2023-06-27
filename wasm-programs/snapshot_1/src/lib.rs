use wasm_bindgen::prelude::*;
mod board;
use crate::board::Board;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn game_start() -> String {
    let mut out = String::new();
    out.push_str("Game started!\n");
    let board = Board::new();
    out.push_str(&format!("{board:?}\n"));
    out
}
