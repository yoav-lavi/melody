# Melody

[![Custom badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fdeno-visualizer.danopia.net%2Fshields%2Flatest-version%2Fx%2Fmelody%2Fmelody_wasm.js)](https://doc.deno.land/https/deno.land/x/melody/melody_wasm.js)

<p align="center">
Deno bindings for the Melody language compiler
</p>

## Usage

```ts
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
```

## Links

- [Language Documentation](https://yoav-lavi.github.io/melody/book/)
- [Repository](https://github.com/yoav-lavi/melody)
