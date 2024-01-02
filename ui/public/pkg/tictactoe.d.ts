/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
*/
export enum BoardState {
  Empty = 0,
  X = 1,
  O = 2,
}
/**
*/
export enum GameEndState {
  Win = 0,
  Tie = 1,
  Ongoing = 2,
}
/**
*/
export enum Agent {
  Player = 0,
  Bot = 1,
}
/**
*/
export class Game {
  free(): void;
/**
* @param {Agent} next_to_move
*/
  constructor(next_to_move: Agent);
/**
* @returns {any[]}
*/
  board(): any[];
/**
* @param {number} input_move
*/
  iter_loop(input_move: number): void;
/**
* @returns {GameEndState}
*/
  is_game_over(): GameEndState;
/**
*/
  display(): void;
/**
*/
  next_to_move: Agent;
/**
*/
  winner?: Agent;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_game_free: (a: number) => void;
  readonly __wbg_get_game_next_to_move: (a: number) => number;
  readonly __wbg_set_game_next_to_move: (a: number, b: number) => void;
  readonly __wbg_get_game_winner: (a: number) => number;
  readonly __wbg_set_game_winner: (a: number, b: number) => void;
  readonly game_new: (a: number) => number;
  readonly game_board: (a: number, b: number) => void;
  readonly game_iter_loop: (a: number, b: number, c: number) => void;
  readonly game_is_game_over: (a: number) => number;
  readonly game_display: (a: number) => void;
  readonly greet: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
