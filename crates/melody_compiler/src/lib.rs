mod ast;
pub mod errors;
mod source;

use ast::ast::to_ast;
use errors::ParseError;
use source::source::to_source;

pub fn compiler(source: &str) -> Result<String, ParseError> {
    let ast = to_ast(source)?;

    let output = to_source(&ast);

    Ok(format!("/{output}/"))
}
