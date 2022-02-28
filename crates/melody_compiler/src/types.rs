use crate::lexer::tokens::Token;

pub type Lexer<'a> = logos::Lexer<'a, Token>;
