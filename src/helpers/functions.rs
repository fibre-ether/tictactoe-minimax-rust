use crate::constants::SHOW_LOG;
use crate::Agent;
use crate::BoardState;

use std::io;

pub fn validate_move(next_move: String, next_valid_moves: Vec<usize>) -> usize {
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

pub fn player_next_move(next_valid_moves: Vec<usize>) -> usize {
    let mut next_move = String::new();
    println!("Enter your move:");
    io::stdin()
        .read_line(&mut next_move)
        .expect("Failed to read move");
    let new_move: usize = validate_move(next_move, next_valid_moves);
    new_move
}

pub fn print_player_message(player_name_option: &Option<Agent>, message: &str) {
    if let Some(player_name) = player_name_option {
        println!("{:?} {}", player_name, message);
    }
}

pub fn are_equal(a: &BoardState, b: &BoardState, c: &BoardState) -> bool {
    a == b && b == c
}

pub fn retrieve_custom_board() -> Vec<BoardState> {
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

pub fn log(log_str: String, depth: isize) {
    if SHOW_LOG {
        for _ in 0..depth {
            print!("- ");
        }
        println!("{log_str}");
    }
}
