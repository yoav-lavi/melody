mod ast;
pub mod errors;
mod source;
mod utils;

use ast::to_ast;
use errors::ParseError;
use source::to_source;
use utils::format_line_comments;

pub fn compiler(source: &str) -> Result<String, ParseError> {
    let formatted_source = format_line_comments(source);

    let ast = to_ast(formatted_source.as_str())?;

    let output = to_source(&ast);

    Ok(format!("/{output}/"))
}
