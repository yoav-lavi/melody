/* tslint:disable */
/* eslint-disable */
/**
 *
 *Compiles Melody source code to a regular expression
 *
 *# Errors
 *
 *Throws an error if a compilation error is encountered
 *
 *# Example
 *
 *```js
 *import init, { compiler } from "https://deno.land/x/melody/melody_wasm.js";
 *
 *await init();
 *
 *const source = `
 *  <start>;
 *
 *  option of "v";
 *
 *  capture major {
 *    some of <digit>;
 *  }
 *
 *  ".";
 *
 *  capture minor {
 *    some of <digit>;
 *  }
 *
 *  ".";
 *
 *  capture patch {
 *    some of <digit>;
 *  }
 *
 *  <end>;
 *`;
 *
 *try {
 *  const output = compiler(source);
 *  new RegExp(output).test("v1.1.1"); // true
 *} catch (error) {
 *  // handle compilation error
 *}
 *```
 * @param {string} source
 * @returns {string}
 */
export function compiler(source: string): string;

export type InitInput =
  | RequestInfo
  | URL
  | Response
  | BufferSource
  | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly compiler: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
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
export default function init(
  module_or_path?: InitInput | Promise<InitInput>
): Promise<InitOutput>;
