pub mod token_formatters;
pub mod tokens;

use self::tokens::Token;
use crate::types::Lexer;
use logos::Logos;

pub fn lex(source: &str) -> Lexer {
    Token::lexer(source)
}
