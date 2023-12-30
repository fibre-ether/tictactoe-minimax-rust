use super::{
    constants::MAX_DEPTH,
    functions::{are_equal, player_next_move, print_player_message},
    structs::{Agent, BoardState, GameEndState, GameOverLine, MiniMaxMode, MiniMaxReturnValue},
};
use crate::helpers::functions::log;
use std::{fmt, time::Instant};
#[derive(Clone)]
pub struct Game {
    pub board: Vec<BoardState>,
    pub player_move: BoardState,
    pub bot_move: BoardState,
    pub next_to_move: Option<Agent>,
}

impl Game {
    pub fn minimax(
        &mut self,
        next_valid_moves: Vec<usize>,
        mode: MiniMaxMode,
        depth: isize,
    ) -> MiniMaxReturnValue {
        log(format!("{}", self), depth);
        match self.is_game_over() {
            GameEndState::Win(_) => {
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
            println!("depth exceeded");

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

    pub fn bot_next_move(&self, next_valid_moves: Vec<usize>) -> usize {
        let mut cloned_game = self.clone();
        let result = cloned_game.minimax(next_valid_moves, MiniMaxMode::Max, 0);
        // println!("{:?}", result);
        result.1.unwrap_or(0)
        // 0
    }

    pub fn retrieve_next_move(&self) -> usize {
        print_player_message(&self.next_to_move, "is playing...");
        let next_valid_moves = self.retrieve_valid_next_moves();
        match self.next_to_move {
            Some(Agent::Player) => player_next_move(next_valid_moves),
            Some(Agent::Bot) => self.bot_next_move(next_valid_moves),
            None => panic!("null agent appeared in retrieve_next_move"),
        }
    }

    pub fn start_loop(&mut self) {
        while self.next_to_move.is_some() {
            let now = Instant::now();
            let next_move = self.retrieve_next_move();
            println!("Played in {} secs", now.elapsed().as_secs_f32());
            self.play_move(next_move);
            // let valid_next_moves = self.retrieve_valid_next_moves();
            println!("{}\n", self);
            // println!("Next valid moves: {:?}", valid_next_moves,);

            match self.is_game_over() {
                GameEndState::Win(winner) => {
                    println!("{:?} wins!", winner);
                    self.next_to_move = None
                }
                GameEndState::Tie => {
                    println!("Game tied");
                    self.next_to_move = None
                }
                GameEndState::Ongoing => self.flip_next_to_move(),
            }
        }
    }

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
