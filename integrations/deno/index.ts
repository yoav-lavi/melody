import init, { compiler as melodyc } from "./wasm/melody_wasm.js";

const initialized = init(Deno.readFile("./wasm/melody_wasm_bg.wasm"));

export async function compiler(source: string): Promise<string> {
  await initialized;
  return melodyc(source);
}
