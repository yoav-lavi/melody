# Melody

<p align="center">
Deno bindings for the Melody language compiler
</p>

## Usage

```ts
import { compiler } from 'https://deno.land/x/melody/melody_wasm.js';

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
  new RegExp(output).test('v1.1.1'); // true
} catch (error) {
  // handle compilation error
}
```

## Links

- [Language Documentation](https://yoav-lavi.github.io/melody/book/)
- [Repository](https://github.com/yoav-lavi/melody)
