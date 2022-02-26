use crate::tokens::Token;
use logos::Lexer;

#[derive(Debug, Clone)]
pub struct ParseError {
    /// the unrecognized token responsible for the [ParseError]
    pub token: String,
    /// the line in which an unrecognized token was encountered
    pub line: String,
    /// 0 based index of the line in which an unrecognized token was encountered
    pub line_index: usize,
}

pub fn create_parse_error(lex: &mut Lexer<Token>, line: u16) -> ParseError {
    let line_index = usize::from(line);
    let line_source = lex.source().split('\n').nth(line_index).unwrap();
    ParseError {
        token: lex.slice().to_owned(),
        line: line_source.to_owned(),
        line_index,
    }
}
