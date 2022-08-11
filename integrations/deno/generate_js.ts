/* Generates Deno distributable melody_wasm_static.js from
 * melody_wasm_bg.wasm
 */

import * as base64 from "https://deno.land/std@0.151.0/encoding/base64.ts";

let lines = `import * as base64 from "https://deno.land/std@0.151.0/encoding/base64.ts";

export const bytes = base64.decode(
	"`;

const bytes = await Deno.readFile("melody_wasm_bg.wasm");
lines += (base64.encode(bytes).match(/.{1,76}/g) ?? []).join("\\\n");

lines += `"
)
`;

await Deno.writeTextFile("./melody_wasm_static.js", lines);
