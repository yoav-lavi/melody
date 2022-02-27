use crate::lexer::tokens::Token;
use crate::lexer::tokens::TokenType;

macro_rules! match_or_err {
    ($condition: expr, $error: literal) => {
        if $condition {
            Ok(())
        } else {
            Err(String::from($error))
        }
    };
}

pub fn allow_next(token: &Token) -> Result<(), String> {
    match_or_err!(
        [TokenType::Newline, TokenType::Semicolon].contains(&token.to_type()),
        "Expected a semicolon or newline"
    )
}

pub fn allow_newline(token: &Token) -> Result<(), String> {
    match_or_err!(token.to_type() == TokenType::Newline, "Expected a newline")
}

pub fn allow_atom(token: &Token) -> Result<(), String> {
    match_or_err!(
        [
            TokenType::Literal,
            TokenType::Group,
            TokenType::Symbol,
            TokenType::Range,
            TokenType::Raw,
            TokenType::Assertion,
        ]
        .contains(&token.to_type()),
        "Expected a literal, raw, group, symbol, or range"
    )
}

pub fn allowed_token(previous: &Option<TokenType>, current: &Token) -> Result<(), String> {
    if let Some(previous) = previous {
        match previous {
            TokenType::Symbol
            | TokenType::Literal
            | TokenType::Raw
            | TokenType::Range
            | TokenType::SpecialSymbol => allow_next(current),

            TokenType::Expression => allow_atom(current),

            TokenType::Group | TokenType::Assertion | TokenType::Other => allow_newline(current),

            // allow any type of token
            TokenType::Semicolon | TokenType::Newline => Ok(()),
        }
    } else {
        Ok(())
    }
}
