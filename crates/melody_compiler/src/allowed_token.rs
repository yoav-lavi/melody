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

static SEPARATOR: [TokenType; 3] = [TokenType::Newline, TokenType::Semicolon, TokenType::Ignored];

static QUANTIFIABLE: [TokenType; 7] = [
    TokenType::Literal,
    TokenType::Group,
    TokenType::Symbol,
    TokenType::Range,
    TokenType::Raw,
    TokenType::Assertion,
    TokenType::Ignored,
];

static BLOCK_END_AND_SEMICOLON: [TokenType; 2] = [TokenType::BlockEnd, TokenType::Semicolon];

pub fn separator(token: &Token) -> Result<(), String> {
    match_or_err!(
        SEPARATOR.contains(&token.to_type()),
        "Expected a semicolon, block end, or newline"
    )
}

pub fn quantifiable(token: &Token) -> Result<(), String> {
    match_or_err!(
        QUANTIFIABLE.contains(&token.to_type()),
        "Expected a quantifiable (literal, raw, group, assertion, symbol, or range)"
    )
}

fn not_block_end_and_semicolon(token: &Token) -> Result<(), String> {
    match_or_err!(
        !BLOCK_END_AND_SEMICOLON.contains(&token.to_type()),
        "Expected a literal, raw, group, assertion, symbol, range, or newline"
    )
}

pub fn allowed_token(previous: &Option<TokenType>, current: &Token) -> Result<(), String> {
    if let Some(previous) = previous {
        match previous {
            TokenType::Symbol
            | TokenType::Literal
            | TokenType::Raw
            | TokenType::Range
            | TokenType::SpecialSymbol => separator(current),

            TokenType::Expression => quantifiable(current),

            TokenType::BlockEnd | TokenType::Semicolon => not_block_end_and_semicolon(current),

            TokenType::Assertion | TokenType::Group | TokenType::Newline | TokenType::Ignored => {
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}
