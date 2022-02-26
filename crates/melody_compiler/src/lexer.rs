pub mod token_formatters;
pub mod tokens;

use self::tokens::Token;
use logos::Logos;

pub fn lex(source: &str) -> logos::Lexer<Token> {
    Token::lexer(source)
}
