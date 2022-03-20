#![forbid(unsafe_code)]
#![warn(clippy::needless_pass_by_value)]

#[cfg(not(feature = "fuzzer"))]
mod ast;
#[cfg(feature = "fuzzer")]
pub mod ast;
pub mod errors;
mod regex;
mod utils;

use ast::to_ast;
use errors::ParseError;
#[cfg(not(feature = "fuzzer"))]
use regex::ast_to_regex;
#[cfg(feature = "fuzzer")]
pub use regex::ast_to_regex::ast_to_regex;
use utils::format_line_comments;

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
pub fn compiler(source: &str) -> Result<String, ParseError> {
    let formatted_source = format_line_comments(source);

    let ast = to_ast(formatted_source.as_str())?;

    let output = ast_to_regex(&ast);

    Ok(output)
}
