import { useEffect, useState } from "react";
import init, { Agent, Game } from "../public/pkg/tictactoe";

export default function UseWasm(startingMove: null | Agent) {
  const [game, setGame] = useState<null | Game>(null);
  useEffect(() => {
    console.log("usewasm useeffect called");
    async function initWasm() {
      await init();
      console.log("init done");
      const wasmGame = startingMove !== null ? new Game(startingMove) : null;
      console.log("game: ", wasmGame);
      if (startingMove === Agent.Bot) {
        wasmGame?.iter_loop(999);
      }
      setGame(wasmGame);
      console.log("game is set");
    }
    startingMove !== null && initWasm();
  }, [startingMove]);

  return game;
}
