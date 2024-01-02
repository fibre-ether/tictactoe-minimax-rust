import { useEffect, useState } from "react";
import { Agent, BoardState, GameEndState } from "../public/pkg/tictactoe";
import UseWasm from "./UseWasm";
import { Circle, X } from "lucide-react";
import "./App.css";

function App() {
  // const [nextMove, setNextMove] = useState<number>(0)
  const [board, setBoard] = useState<number[]>([]);
  const [refetch, setRefetch] = useState<number>(0);
  const [startingMove, setStartingMove] = useState<null | Agent>(null);
  const [gameOver, setGameOver] = useState<null | string>(null);
  const game = UseWasm(startingMove);

  useEffect(() => {
    if (game) {
      setBoard(game?.board());
      handleGameOver();
    }
  }, [game, refetch]);

  function handleGameOver() {
    switch (game?.is_game_over()) {
      case GameEndState.Tie:
        setGameOver("Game tied");
        setStartingMove(null);
        break;

      case GameEndState.Win:
        console.log(game.winner);
        setGameOver(`${Agent[game?.winner]} wins!`);
        setStartingMove(null);
        break;

      case GameEndState.Ongoing:
        setGameOver(null);
        console.log("game is ongoing");
        break;
    }
  }

  function handleClick(index: number) {
    console.log("in handleclick");
    if (game?.is_game_over() === GameEndState.Ongoing) {
      if (board[index] === 0) {
        console.log("player is playing");
        game.iter_loop(index);
        console.log("bot is playing");
        game.iter_loop(999);
      }
    }
    setRefetch((state) => state + 1);
  }

  function retrieveSymbol(state: BoardState) {
    switch (state) {
      case BoardState.Empty:
        return " ";
      case BoardState.O:
        return <Circle />;
      case BoardState.X:
        return <X size={35} strokeWidth={1.5} />;
    }
  }

  function handleStartingMove(move: Agent) {
    if (game?.is_game_over() !== GameEndState.Ongoing) {
      setStartingMove(move);
    }
  }

  return (
    <div className="h-screen w-screen flex flex-col items-center justify-center space-y-4 font-semibold">
      <div className="flex flex-col items-center space-y-2">
        <p className="text-2xl">Start as</p>
        <div className="space-x-4 flex">
          <button
            className={`${
              startingMove === Agent.Player
                ? "border-blue-300 bg-blue-100"
                : "border-blue-200 bg-slate-200"
            } rounded-lg border-4 py-1 px-2 flex items-center space-x-2`}
            onClick={() => handleStartingMove(Agent.Player)}>
            <p>Player </p>
            <X size={35} strokeWidth={1.5} />
          </button>
          <button
            className={`${
              startingMove === Agent.Bot
                ? "border-blue-300 bg-blue-100"
                : "border-blue-200 bg-slate-200"
            } rounded-lg border-4 py-1 px-2 flex items-center space-x-2`}
            onClick={() => handleStartingMove(Agent.Bot)}>
            <p>Bot</p>
            <Circle />
          </button>
        </div>
      </div>
      <div className="grid grid-cols-3 grid-rows-3 gap-3">
        {board.map((item, index) => {
          return (
            <button
              onClick={() => {
                game && handleClick(index);
              }}
              key={index}
              className=" bg-blue-100 h-12 w-12 border-4 border-blue-300 rounded-md flex justify-center items-center text-lg">
              <p>{retrieveSymbol(item)}</p>
            </button>
          );
        })}
      </div>
      <p className="text-xl h-12">{gameOver}</p>
      <div className="attributions underline text-white">
        <a
          href="https://github.com/fibre-ether"
          target="_blank"
          rel={"noreferrer noopener"}>
          @fibre-ether
        </a>
        <a
          href="https://github.com/fibre-ether/wasm-tictactoe"
          target="_blank"
          rel={"noreferrer noopener"}>
          Code
        </a>
      </div>
    </div>
  );
}

export default App;
