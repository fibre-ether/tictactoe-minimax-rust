mod helpers;
use crate::helpers::{
    constants,
    game::Game,
    structs::{Agent, BoardState},
};

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
    println!("{}\n", game);
    game.start_loop();
    // }
}
