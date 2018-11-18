extern crate wasm_bindgen;
extern crate binoxxo;

use wasm_bindgen::prelude::*;
use binoxxo::rules::is_board_valid;
use std::str::FromStr;
pub use binoxxo::field::Board;

#[wasm_bindgen]
pub fn check_board(board: &str) -> bool {
    let board = Board::from_str(board);
    if let Ok(board) = board {
        is_board_valid(&board)
    }
    else {
        false
    }
}

pub const BOARD_SIZE : usize = 10;

#[wasm_bindgen]
pub fn create_puzzle(level: usize) -> String {
    let board = binoxxo::bruteforce::create_puzzle_board(BOARD_SIZE, level);

    board.to_string()
}
