#![feature(pattern)]

extern crate binoxxo_wasm;
extern crate wasm_bindgen_test;

use binoxxo_wasm::{create_puzzle, Board, BOARD_SIZE};
use std::str::FromStr;
use std::str::pattern::Pattern;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn create_puzzle_returns_a_board_of_correct_size() {
    let level = 15;

    let board_str = create_puzzle(level);
    let board = Board::from_str(&board_str).unwrap();

    assert_eq!(BOARD_SIZE, board.get_size());
}

#[wasm_bindgen_test]
fn create_puzzle_returns_incomplete_board() {
    let level = 15;

    let board_str = create_puzzle(level);

    assert!("_".is_contained_in(&board_str));
}