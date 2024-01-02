use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use super::{
    constants::MAX_DEPTH,
    functions::{are_equal, player_next_move},
    structs::{Agent, BoardState, GameEndState, GameOverLine, MiniMaxMode, MiniMaxReturnValue},
};
use crate::helpers::functions::log;
use std::fmt;

#[wasm_bindgen]
pub struct Game {
    board: Vec<BoardState>,
    player_move: BoardState,
    bot_move: BoardState,
    pub next_to_move: Agent,
    pub winner: Option<Agent>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(next_to_move: Agent) -> Game {
        let mut board: Vec<BoardState> = Vec::new();
        for _ in 0..9 {
            board.push(BoardState::Empty);
        }

        Game {
            board,
            player_move: BoardState::X,
            bot_move: BoardState::O,
            next_to_move,
            winner: None,
        }
    }

    #[wasm_bindgen(method)]
    pub fn board(&self) -> Vec<BoardState> {
        self.board.to_vec()
    }

    #[wasm_bindgen(method)]
    pub fn iter_loop(&mut self, input_move: usize) -> Result<(), JsValue> {
        if self.is_game_over() == GameEndState::Ongoing {
            let next_move = self.retrieve_next_move(input_move)?;
            self.play_move(next_move);

            if let GameEndState::Ongoing = self.is_game_over() {
                self.flip_next_to_move()
            }
        }
        Ok(())
    }

    #[wasm_bindgen(method)]
    pub fn is_game_over(&mut self) -> GameEndState {
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
                let winner = self.next_to_move;
                self.winner = Some(winner);
                return GameEndState::Win;
            }
        }
        if self.retrieve_valid_next_moves().is_empty() {
            return GameEndState::Tie;
        }
        GameEndState::Ongoing
    }

    pub fn display(&self) {
        println!("{}\n", self);
    }

    fn minimax(
        &mut self,
        next_valid_moves: Vec<usize>,
        mode: MiniMaxMode,
        depth: isize,
    ) -> MiniMaxReturnValue {
        log(format!("{}", self), depth);
        match self.is_game_over() {
            GameEndState::Win => {
                log("game over with win".to_string(), depth);
                return match mode {
                    MiniMaxMode::Max => MiniMaxReturnValue(depth + -10, None),
                    MiniMaxMode::Min => MiniMaxReturnValue(-depth + 10, None),
                };
            }

            GameEndState::Tie => {
                log("game over with tie".to_string(), depth);
                return MiniMaxReturnValue(0, None);
            }
            GameEndState::Ongoing => (),
        }
        if depth > MAX_DEPTH {
            return MiniMaxReturnValue(depth, None);
        }

        let mut scores: Vec<isize> = Vec::new();
        for valid_move in &next_valid_moves {
            log(format!("{:?} playing move {}", mode, valid_move), depth);
            self.play_move(*valid_move);
            let next_next_valid_moves = self.retrieve_valid_next_moves();
            self.flip_next_to_move();
            let new_score = self
                .minimax(next_next_valid_moves, mode.flip(), depth + 1)
                .0;
            self.flip_next_to_move();
            self.rewind(*valid_move);
            scores.push(new_score);
        }

        log(format!("valid moves: {:?}", next_valid_moves), depth);
        log(format!("scores: {:?}", scores), depth);
        match mode {
            MiniMaxMode::Max => {
                let result = scores
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .unwrap_or((0, &0));
                MiniMaxReturnValue(*result.1, Some(next_valid_moves[result.0]))
            }
            MiniMaxMode::Min => {
                let result = scores
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.cmp(b))
                    .unwrap_or((0, &0));
                MiniMaxReturnValue(*result.1, Some(result.0))
            }
        }
    }

    fn bot_next_move(&mut self, next_valid_moves: Vec<usize>) -> usize {
        let result = self.minimax(next_valid_moves, MiniMaxMode::Max, 0);
        result.1.unwrap_or(0)
    }

    fn retrieve_next_move(&mut self, input_move: usize) -> Result<usize, JsValue> {
        let next_valid_moves = self.retrieve_valid_next_moves();
        match self.next_to_move {
            Agent::Player => player_next_move(next_valid_moves, input_move),
            Agent::Bot => Ok(self.bot_next_move(next_valid_moves)),
        }
    }

    fn flip_next_to_move(&mut self) {
        match self.next_to_move {
            Agent::Bot => self.next_to_move = Agent::Player,
            Agent::Player => self.next_to_move = Agent::Bot,
        }
    }

    fn retrieve_symbol(&self) -> BoardState {
        match self.next_to_move {
            Agent::Bot => BoardState::new(&self.bot_move),
            Agent::Player => BoardState::new(&self.player_move),
        }
    }

    fn play_move(&mut self, next_move: usize) {
        self.board[next_move] = self.retrieve_symbol();
    }

    fn rewind(&mut self, played_move: usize) {
        self.board[played_move] = BoardState::Empty;
    }

    fn retrieve_valid_next_moves(&self) -> Vec<usize> {
        let mut valid_next_moves: Vec<usize> = Vec::new();
        for (index, item) in self.board.iter().enumerate() {
            if let BoardState::Empty = item {
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
