#![forbid(unsafe_code)]
#![allow(clippy::unused_unit)]
#![allow(clippy::module_name_repetitions)]

use wasm_bindgen::prelude::*;

/**
Compiles Melody source code to a regular expression

# Errors

Throws an error if a compilation error is encountered

# Example

```js
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
*/
#[wasm_bindgen]
pub fn compiler(source: &str) -> Result<String, JsError> {
    let output = melody_compiler::compiler(source);
    output.map_err(|error| JsError::new(&error.to_string()))
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
