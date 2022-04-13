import init, { compiler } from "https://deno.land/x/melody/melody_wasm.js";

await init();

const source = `

  <start>;

  option of "v";

  capture major {

    some of <digit>;

  }

  ".";

  capture minor {

    some of <digit>;

  }

  ".";

  capture patch {

    some of <digit>;

  }

  <end>;

`;

try {

  const output = compiler(source);

  new RegExp(output).test("v1.1.1"); // true

} catch (error: unknown) {

  // handle compilation error

}
