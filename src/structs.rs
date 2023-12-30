use std::fmt;

use crate::are_equal;

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

#[derive(Clone)]
pub struct Game {
    pub board: Vec<BoardState>,
    pub player_move: BoardState,
    pub bot_move: BoardState,
    pub next_to_move: Option<Agent>,
}

impl Game {
    pub fn flip_next_to_move(&mut self) {
        match self.next_to_move {
            Some(Agent::Bot) => self.next_to_move = Some(Agent::Player),
            Some(Agent::Player) => self.next_to_move = Some(Agent::Bot),
            None => panic!("Cannot flip a null player"),
        }
    }

    fn retrieve_symbol(&self) -> BoardState {
        match self.next_to_move {
            Some(Agent::Bot) => self.bot_move.clone(),
            Some(Agent::Player) => self.player_move.clone(),
            None => panic!("this was never supposed to happen"),
        }
    }

    pub fn play_move(&mut self, next_move: usize) {
        self.board[next_move] = self.retrieve_symbol();
    }

    pub fn rewind(&mut self, played_move: usize) {
        self.board[played_move] = BoardState::Empty(played_move);
    }

    pub fn is_game_over(&self) -> GameEndState {
        let game_over_lines = vec![
            GameOverLine(0, 1, 2),
            GameOverLine(3, 4, 5),
            GameOverLine(6, 7, 8),
            GameOverLine(0, 3, 6),
            GameOverLine(1, 4, 7),
            GameOverLine(2, 5, 8),
            GameOverLine(0, 4, 8),
            GameOverLine(2, 4, 6),
        ];
        let board = &self.board;

        for gol in game_over_lines {
            if are_equal(&board[gol.0], &board[gol.1], &board[gol.2]) {
                return GameEndState::Win(self.next_to_move.clone().unwrap().clone());
            }
        }
        if self.retrieve_valid_next_moves().is_empty() {
            return GameEndState::Tie;
        }
        GameEndState::Ongoing
    }

    pub fn retrieve_valid_next_moves(&self) -> Vec<usize> {
        let mut valid_next_moves: Vec<usize> = Vec::new();
        for (index, item) in self.board.iter().enumerate() {
            if let BoardState::Empty(_) = item {
                valid_next_moves.push(index);
            }
        }
        valid_next_moves
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let board = &self.board;
        write!(
            f,
            "{} {} {}\n{} {} {}\n{} {} {}",
            board[0],
            board[1],
            board[2],
            board[3],
            board[4],
            board[5],
            board[6],
            board[7],
            board[8]
        )
    }
}
