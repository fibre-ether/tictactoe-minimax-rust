use std::fmt;

use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub enum GameEndState {
    Win,
    Tie,
    Ongoing,
}

#[derive(Debug)]
pub struct MiniMaxReturnValue(pub isize, pub Option<usize>);
#[derive(Debug)]
pub enum MiniMaxMode {
    Max,
    Min,
}

impl MiniMaxMode {
    pub fn flip(&self) -> MiniMaxMode {
        match self {
            MiniMaxMode::Max => MiniMaxMode::Min,
            MiniMaxMode::Min => MiniMaxMode::Max,
        }
    }
}

pub struct GameOverLine(pub usize, pub usize, pub usize);

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Agent {
    Player,
    Bot,
}

#[derive(Clone)]
#[wasm_bindgen]
pub enum BoardState {
    Empty,
    X,
    O,
}

impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BoardState::X, BoardState::X) => true,
            (BoardState::O, BoardState::O) => true,
            (BoardState::Empty, BoardState::Empty) => false,
            _ => false,
        }
    }
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("-"),
            Self::X => f.write_str("x"),
            Self::O => f.write_str("o"),
        }
    }
}

impl BoardState {
    pub fn new(to_clone: &BoardState) -> BoardState {
        match to_clone {
            BoardState::Empty => BoardState::Empty,
            BoardState::X => BoardState::X,
            BoardState::O => BoardState::O,
        }
    }
}
