#![forbid(unsafe_code)]
#![warn(clippy::needless_pass_by_value)]

#[cfg(not(feature = "fuzzer"))]
mod ast;
#[cfg(feature = "fuzzer")]
pub mod ast;
pub mod errors;
mod format;
mod regex;

use anyhow::Result;
use ast::to_ast;
use format::format;
#[cfg(not(feature = "fuzzer"))]
use regex::ast_to_regex;
#[cfg(feature = "fuzzer")]
pub use regex::ast_to_regex::ast_to_regex;

/**
Compiles Melody source code to a regular expression.

# Errors

Returns a [`ParseError`] upon encountering a syntax error

# Example

```rust
use melody_compiler::compiler;

let source = r#"1 to 5 of "A";"#;
let output = compiler(source);

assert_eq!(output.unwrap(), "A{1,5}");
```
*/
pub fn compiler(source: &str) -> Result<String> {
    let formatted_source = format(source);

    let ast = to_ast(&formatted_source)?;

    let output = ast_to_regex(&ast);

    Ok(output)
}
