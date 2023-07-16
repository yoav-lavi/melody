#![forbid(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

#[cfg(not(feature = "fuzzer"))]
mod ast;
#[cfg(feature = "fuzzer")]
pub mod ast;
pub mod errors;
mod format;
mod regex;
mod types;

use ast::to_ast;
use format::format;
#[cfg(not(feature = "fuzzer"))]
use regex::ast_to_regex;
#[cfg(feature = "fuzzer")]
pub use regex::ast_to_regex::ast_to_regex;
use types::Result;

/**
Compiles Melody source code to a regular expression.

# Errors

Returns an [`errors::CompilerError`] upon encountering a syntax error

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
