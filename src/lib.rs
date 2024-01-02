mod helpers;

use std::io;

use crate::helpers::{
    constants,
    game::Game,
    structs::{Agent, BoardState},
};

use helpers::structs::GameEndState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name));
}

fn user_input(move_var: &mut usize) -> Result<(), &'static str> {
    let mut new_move = String::new();
    println!("Enter your move:");
    io::stdin()
        .read_line(&mut new_move)
        .expect("Failed to read move");

    match new_move.trim().parse::<usize>() {
        Ok(value) => {
            *move_var = value;
            Ok(())
        }
        Err(_) => Err("Could not parse"),
    }
}

fn main() {
    let mut game = Game::new(Agent::Player);
    let mut next_move: usize = 0;
    game.display();
    // println!("{:?}", game.is_game_over());
    while game.is_game_over() == GameEndState::Ongoing {
        if match game.next_to_move {
            Agent::Player => user_input(&mut next_move),
            _ => Ok(()),
        }
        .is_err()
        {
            continue;
        };
        if game.iter_loop(next_move).is_err() {
            continue;
        }
        game.display();
    }
    match game.is_game_over() {
        GameEndState::Win => {
            println!("{:?} wins!", game.winner);
        }
        GameEndState::Tie => {
            println!("Game tied");
        }
        _ => (),
    }
}
