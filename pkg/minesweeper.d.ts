/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
* @param {number} x
* @param {number} y
* @param {number} nb
*/
export function createMinesweeper(x: number, y: number, nb: number): void;
/**
* @returns {string}
*/
export function getState(): string;
/**
* @param {number} x
* @param {number} y
*/
export function openField(x: number, y: number): void;
/**
* @param {number} x
* @param {number} y
*/
export function toggleFlag(x: number, y: number): void;
/**
* @returns {number}
*/
export function getStatus(): number;
/**
*/
export enum StatusGame {
  FirstOpening,
  Playing,
  Lost,
  Win,
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly greet: (a: number, b: number) => void;
  readonly createMinesweeper: (a: number, b: number, c: number) => void;
  readonly getState: (a: number) => void;
  readonly openField: (a: number, b: number) => void;
  readonly toggleFlag: (a: number, b: number) => void;
  readonly getStatus: () => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
