#![feature(pattern)]

extern crate wasm_bindgen_test;
extern crate binoxxo_wasm;

use binoxxo_wasm::check_board;
use std::str::FromStr;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn check_board_checks_correct_board() {
    assert_eq!(true, check_board("
        X O X O
        O X O X
        X X O O
        O O X X"));
}

#[wasm_bindgen_test]
fn check_board_checks_incorrect_board() {
    assert_eq!(false, check_board("
        X X X O
        O X O X
        X X O O
        O O X X"));
}

#[wasm_bindgen_test]
fn check_board_checks_incomplete_board() {
    assert_eq!(false, check_board("
        X _ X O
        O X O X
        X X O O
        O O X X"));
}

#[wasm_bindgen_test]
fn check_board_checks_something_that_is_no_board() {
    assert_eq!(false, check_board("
        O hello X X"));
}
