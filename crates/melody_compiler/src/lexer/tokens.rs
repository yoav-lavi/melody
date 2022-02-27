use super::token_formatters::{
    literal, named_capture, open_range_expression, quantifier, range, range_expression, raw,
};
use logos::Logos;

#[derive(Logos, Debug)]
pub enum Token {
    #[regex(r#"\d+ to \d+ of"#, range_expression)]
    RangeExpression(String),

    #[regex(r#"\d+ of"#, quantifier)]
    QuantifierExpression(String),

    #[regex(r#"over \d+ of"#, open_range_expression)]
    OpenRangeExpression(String),

    #[regex("some of")]
    SomeExpression,

    #[regex("option of")]
    OptionExpression,

    #[regex("any of")]
    AnyExpression,

    #[regex(r#""(\\"|[^"\n])*""#, literal)]
    LiteralDouble(String),

    #[regex(r#"'(\\'|[^'\n])*'"#, literal)]
    LiteralSingle(String),

    #[regex(r#"`(\\`|[^`\n])*`"#, raw)]
    Raw(String),

    #[regex("[a-z] to [a-z]", range)]
    LowercaseRange(String),

    #[regex("[A-Z] to [A-Z]", range)]
    UppercaseRange(String),

    #[regex("[0-9] to [0-9]", range)]
    NumericRange(String),

    #[regex(r#"capture \w+ \{"#, named_capture)]
    NamedCapture(String),

    #[token("capture {")]
    Capture,

    #[token("match {")]
    Match,

    #[token("either {")]
    Either,

    #[token("ahead {")]
    Ahead,

    #[token("behind {")]
    Behind,

    #[token("not ahead {")]
    NotAhead,

    #[token("not behind {")]
    NotBehind,

    #[token("}")]
    BlockEnd,

    #[token("<start>")]
    StartSymbol,

    #[token("<end>")]
    EndSymbol,

    #[token("<newline>")]
    NewlineSymbol,

    #[token("<tab>")]
    TabSymbol,

    #[token("<return>")]
    ReturnSymbol,

    #[token("<feed>")]
    FeedSymbol,

    #[token("<null>")]
    NullSymbol,

    #[token("<digit>")]
    DigitSymbol,

    #[token("not <digit>")]
    NotDigitSymbol,

    #[token("<whitespace>")]
    WhitespaceSymbol,

    #[token("not <whitespace>")]
    NotWhitespaceSymbol,

    #[token("<space>")]
    SpaceSymbol,

    #[token("<word>")]
    WordSymbol,

    #[token("not <word>")]
    NotWordSymbol,

    #[token("<vertical>")]
    VerticalSymbol,

    #[token("<alphabet>")]
    AlphabetSymbol,

    #[token("<char>")]
    CharSymbol,

    #[token(";")]
    Semicolon,

    #[regex(r"\n")]
    Newline,

    #[regex("//.*", logos::skip)]
    Comment,

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    Unidentified,
}

#[derive(PartialEq, Eq)]
pub enum TokenType {
    Symbol,
    Literal,
    Raw,
    Expression,
    Range,
    Group,
    Assertion,
    SpecialSymbol,
    Semicolon,
    Newline,
    Other,
}

impl Token {
    pub fn to_type(&self) -> TokenType {
        match *self {
            Token::RangeExpression(_) => TokenType::Expression,
            Token::QuantifierExpression(_) => TokenType::Expression,
            Token::OpenRangeExpression(_) => TokenType::Expression,
            Token::SomeExpression => TokenType::Expression,
            Token::OptionExpression => TokenType::Expression,
            Token::AnyExpression => TokenType::Expression,
            Token::LiteralDouble(_) => TokenType::Literal,
            Token::LiteralSingle(_) => TokenType::Literal,
            Token::Raw(_) => TokenType::Raw,
            Token::LowercaseRange(_) => TokenType::Range,
            Token::UppercaseRange(_) => TokenType::Range,
            Token::NumericRange(_) => TokenType::Range,
            Token::NamedCapture(_) => TokenType::Range,
            Token::Capture => TokenType::Group,
            Token::Match => TokenType::Group,
            Token::Either => TokenType::Group,
            Token::Ahead => TokenType::Assertion,
            Token::Behind => TokenType::Assertion,
            Token::NotAhead => TokenType::Assertion,
            Token::NotBehind => TokenType::Assertion,
            Token::BlockEnd => TokenType::Other,
            Token::StartSymbol => TokenType::SpecialSymbol,
            Token::EndSymbol => TokenType::SpecialSymbol,
            Token::NewlineSymbol => TokenType::Symbol,
            Token::TabSymbol => TokenType::Symbol,
            Token::ReturnSymbol => TokenType::Symbol,
            Token::FeedSymbol => TokenType::Symbol,
            Token::NullSymbol => TokenType::Symbol,
            Token::DigitSymbol => TokenType::Symbol,
            Token::NotDigitSymbol => TokenType::Symbol,
            Token::WhitespaceSymbol => TokenType::Symbol,
            Token::NotWhitespaceSymbol => TokenType::Symbol,
            Token::SpaceSymbol => TokenType::Symbol,
            Token::WordSymbol => TokenType::Symbol,
            Token::NotWordSymbol => TokenType::Symbol,
            Token::VerticalSymbol => TokenType::Symbol,
            Token::AlphabetSymbol => TokenType::Symbol,
            Token::CharSymbol => TokenType::Symbol,
            Token::Semicolon => TokenType::Semicolon,
            Token::Newline => TokenType::Newline,
            Token::Comment => TokenType::Other,
            Token::Unidentified => TokenType::Other,
        }
    }
}
