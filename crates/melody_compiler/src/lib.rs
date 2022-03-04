#![forbid(unsafe_code)]

mod ast;
pub mod errors;
mod source;
mod utils;

use ast::to_ast;
use errors::ParseError;
use source::to_source;
use utils::format_line_comments;

/**
Compiles Melody source code to a regular expression.

see also: [`ParseError`]

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

    let output = to_source(&ast);

    Ok(output)
}
