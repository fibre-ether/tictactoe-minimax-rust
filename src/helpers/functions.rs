use wasm_bindgen::JsValue;

use crate::constants::SHOW_LOG;
use crate::BoardState;

pub fn player_next_move(next_valid_moves: Vec<usize>, input_move: usize) -> Result<usize, JsValue> {
    if next_valid_moves.contains(&input_move) {
        Ok(input_move)
    } else {
        Err(JsValue::NULL)
    }
}

pub fn are_equal(a: &BoardState, b: &BoardState, c: &BoardState) -> bool {
    a == b && b == c
}

pub fn retrieve_custom_board() -> Vec<BoardState> {
    vec![
        BoardState::X,
        BoardState::Empty,
        BoardState::Empty,
        BoardState::X,
        BoardState::O,
        BoardState::Empty,
        BoardState::Empty,
        BoardState::Empty,
        BoardState::O,
    ]
}

pub fn log(log_str: String, depth: isize) {
    if SHOW_LOG {
        for _ in 0..depth {
            print!("- ");
        }
        println!("{log_str}");
    }
}
