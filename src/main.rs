mod structs;
use std::{io, vec};

use structs::{MiniMaxMode, MiniMaxReturnValue};

use crate::structs::{Agent, BoardState, Game, GameEndState};

const MAX_DEPTH: isize = 99;
const SHOW_LOG: bool = false;

fn validate_move(next_move: String, next_valid_moves: Vec<usize>) -> usize {
    match next_move.trim().parse::<usize>() {
        Ok(value) => {
            if next_valid_moves.contains(&value) {
                value
            } else {
                println!(
                    "Entered value ({}) is not valid (not in valid moves)",
                    value
                );
                player_next_move(next_valid_moves)
            }
        }
        Err(_) => {
            println!(
                "Entered value ({}) is not valid (could not parse)",
                next_move.trim()
            );
            player_next_move(next_valid_moves)
        }
    }
}

fn player_next_move(next_valid_moves: Vec<usize>) -> usize {
    let mut next_move = String::new();
    println!("Enter your move:");
    io::stdin()
        .read_line(&mut next_move)
        .expect("Failed to read move");
    let new_move: usize = validate_move(next_move, next_valid_moves);
    new_move
}

fn minimax(
    game: &mut Game,
    next_valid_moves: Vec<usize>,
    mode: MiniMaxMode,
    depth: isize,
) -> MiniMaxReturnValue {
    // return mini max return value formatk
    // println!("game in minimax\n{}", game);
    // println!("valid moves: {:?}", next_valid_moves);
    log(format!("{}", game), depth);

    match game.is_game_over() {
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
    // println!("depth: {depth}");

    let mut scores: Vec<isize> = Vec::new();
    for valid_move in &next_valid_moves {
        log(format!("{:?} playing move {}", mode, valid_move), depth);
        game.play_move(*valid_move);
        let next_next_valid_moves = game.retrieve_valid_next_moves();
        game.flip_next_to_move();
        let new_score = minimax(game, next_next_valid_moves, mode.flip(), depth + 1).0;
        game.flip_next_to_move();
        game.rewind(*valid_move);
        scores.push(new_score);
    }

    // println!("scores: {:?}", scores);
    // println!(
    //     "max: {:?}",
    //     scores.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)) // .map(|(idx, _)| idx)
    // );

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

fn bot_next_move(game: &Game, next_valid_moves: Vec<usize>) -> usize {
    let mut cloned_game = game.clone();
    let result = minimax(&mut cloned_game, next_valid_moves, MiniMaxMode::Max, 0);
    // println!("{:?}", result);
    result.1.unwrap_or(0)
    // 0
}

fn print_player_message(player_name_option: &Option<Agent>, message: &str) {
    if let Some(player_name) = player_name_option {
        println!("{:?} {}", player_name, message);
    }
}

fn retrieve_next_move(game: &Game) -> usize {
    print_player_message(&game.next_to_move, "is playing...");
    let next_valid_moves = game.retrieve_valid_next_moves();
    match game.next_to_move {
        Some(Agent::Player) => player_next_move(next_valid_moves),
        Some(Agent::Bot) => bot_next_move(game, next_valid_moves),
        None => panic!("this was never supposed to happen"),
    }
}

fn are_equal(a: &BoardState, b: &BoardState, c: &BoardState) -> bool {
    a == b && b == c
}

fn game_loop(game: &mut Game) {
    while game.next_to_move.is_some() {
        let next_move = retrieve_next_move(game);
        game.play_move(next_move);

        let valid_next_moves = game.retrieve_valid_next_moves();
        println!("{}", game);
        println!("Next valid moves: {:?}", valid_next_moves,);
        // if valid_next_moves.is_empty() {
        //     println!("Game tied");
        //     game.next_to_move = None
        match game.is_game_over() {
            GameEndState::Win(winner) => {
                println!("{:?} wins!", winner);
                game.next_to_move = None
            }
            GameEndState::Tie => {
                println!("Game tied");
                game.next_to_move = None
            }
            GameEndState::Ongoing => game.flip_next_to_move(),
        }
    }
}

fn retrieve_custom_board() -> Vec<BoardState> {
    vec![
        BoardState::X,
        BoardState::Empty(1),
        BoardState::Empty(2),
        BoardState::X,
        BoardState::O,
        BoardState::Empty(5),
        BoardState::Empty(6),
        BoardState::Empty(7),
        BoardState::O,
    ]
}

fn log(log_str: String, depth: isize) {
    if SHOW_LOG {
        for _ in 0..depth {
            print!("- ");
        }
        println!("{log_str}");
    }
}

fn main() {
    // loop {
    // let board = retrieve_custom_board();
    let mut board: Vec<BoardState> = Vec::new();
    for index in 0..9 {
        board.push(BoardState::Empty(index));
    }
    let mut game = Game {
        board,
        player_move: BoardState::X,
        bot_move: BoardState::O,
        next_to_move: Some(Agent::Bot),
    };
    println!("{}", game);
    game_loop(&mut game);
    // }
}
