import init, { Game, Agent,  } from "./tictactoe.js";

init().then(()=>{
    console.log("hello from init")
    const next_to_move = Agent.Bot
    const game = new Game(next_to_move)
    console.log(game.board())
    game.winner = Agent.Bot
    console.log(game.winner)
    console.log(game.is_game_over())
    game.iter_loop(1)
    console.log(game.board())
})