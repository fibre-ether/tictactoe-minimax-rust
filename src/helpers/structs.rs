use std::fmt;

pub enum GameEndState {
    Win(Agent),
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

#[derive(Debug, Clone)]
pub enum Agent {
    Player,
    Bot,
}

#[derive(Clone, PartialEq)]
pub enum BoardState {
    Empty(usize),
    X,
    O,
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty(index) => write!(f, "{}", index),
            Self::X => f.write_str("x"),
            Self::O => f.write_str("o"),
        }
    }
}
