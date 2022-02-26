use super::token_formatters::{
    literal, named_capture, open_range_expression, quantifier, range, range_expression, raw,
};
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
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

    #[token("not <space>")]
    NotSpaceSymbol,

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
    NewLine,

    #[regex("//.*", logos::skip)]
    Comment,

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    Unidentified,
}
