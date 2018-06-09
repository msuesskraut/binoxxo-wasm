#![feature(proc_macro, wasm_custom_section, wasm_import_module, pattern)]

extern crate wasm_bindgen;
extern crate binoxxo;

use wasm_bindgen::prelude::*;
use binoxxo::field::Board;
use binoxxo::rules::is_board_valid;
use std::str::FromStr;

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

const BOARD_SIZE : usize = 10;

#[wasm_bindgen]
pub fn create_puzzle(level: usize) -> String {
    let board = binoxxo::bruteforce::create_puzzle_board(BOARD_SIZE, level);

    board.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::pattern::Pattern;

    #[test]
    fn check_board_checks_correct_board() {
        assert_eq!(true, check_board("
            X O X O
            O X O X
            X X O O
            O O X X"));
    }

    #[test]
    fn check_board_checks_incorrect_board() {
        assert_eq!(false, check_board("
            X X X O
            O X O X
            X X O O
            O O X X"));
    }

    #[test]
    fn check_board_checks_incomplete_board() {
        assert_eq!(false, check_board("
            X _ X O
            O X O X
            X X O O
            O O X X"));
    }

    #[test]
    fn check_board_checks_something_that_is_no_board() {
        assert_eq!(false, check_board("
            O hello X X"));
    }

    #[test]
    fn create_puzzle_returns_a_board_of_correct_size() {
        let level = 15;

        let board_str = create_puzzle(level);
        let board = Board::from_str(&board_str).unwrap();

        assert_eq!(BOARD_SIZE, board.get_size());
    }

    #[test]
    fn create_puzzle_returns_incomplete_board() {
        let level = 15;

        let board_str = create_puzzle(level);

        assert!("_".is_contained_in(&board_str));
    }
}
