<p align="center">
    <img alt="Melody Logo" height="250px" src="https://user-images.githubusercontent.com/14347895/159069181-53bce5b3-a831-43f1-8c14-af6c6ed7b92b.svg">
</p>

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
} catch (error) {
  // handle compilation error
}
```

## Links

- [Language Documentation](https://yoav-lavi.github.io/melody/book/)
- [Repository](https://github.com/yoav-lavi/melody)