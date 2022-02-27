use crate::lexer::tokens::Token;
use crate::lexer::tokens::TokenType;

pub fn allow_next(token: &Token) -> Result<(), String> {
    if [TokenType::Newline, TokenType::Semicolon].contains(&token.to_type()) {
        Ok(())
    } else {
        Err(String::from("expected a semicolon or newline"))
    }
}

pub fn allow_newline(token: &Token) -> Result<(), String> {
    if token.to_type() == TokenType::Newline {
        Ok(())
    } else {
        Err(String::from("expected a newline"))
    }
}

pub fn allow_atom(token: &Token) -> Result<(), String> {
    if [
        TokenType::Literal,
        TokenType::Group,
        TokenType::Symbol,
        TokenType::Range,
        TokenType::Raw,
        TokenType::Assertion,
    ]
    .contains(&token.to_type())
    {
        Ok(())
    } else {
        Err(String::from(
            "expected a literal, raw, group, symbol, or range",
        ))
    }
}

pub fn allowed_token(previous: &Option<TokenType>, current: &Token) -> Result<(), String> {
    if let Some(previous) = previous {
        match previous {
            TokenType::Symbol => allow_next(current),
            TokenType::Literal => allow_next(current),
            TokenType::Raw => allow_next(current),
            TokenType::Expression => allow_atom(current),
            TokenType::Range => allow_next(current),
            TokenType::Group => allow_newline(current),
            TokenType::Assertion => allow_newline(current),
            TokenType::SpecialSymbol => allow_next(current),
            TokenType::Semicolon => Ok(()),
            TokenType::Newline => Ok(()),
            TokenType::Other => allow_newline(current),
        }
    } else {
        return Ok(());
    }
}
