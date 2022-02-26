use logos::{Lexer, Logos};
use std::collections::HashSet;

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

enum QuoteType {
    Single,
    Double,
    Raw,
}

fn quantifier(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let amount: u16 = slice[..slice.len() - 3].parse().ok()?;
    Some(format!("{{{amount}}}"))
}

fn named_capture(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    slice[8..slice.len() - 2].to_owned()
}

fn range_expression(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    let range: &str = &slice[..slice.len() - 3];
    let formatted_range = range.replace(" to ", ",");
    format!("{{{formatted_range}}}")
}

fn open_range_expression(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let range: i32 = slice[5..slice.len() - 3].parse().ok()?;
    let incremented_range = range + 1;

    Some(format!("{{{incremented_range},}}"))
}

fn range(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    let formatted_slice = slice.replace(" to ", "-");
    format!("[{formatted_slice}]")
}

fn get_quote_type(quote: &str) -> QuoteType {
    if quote == "\"" {
        QuoteType::Double
    } else {
        QuoteType::Single
    }
}

fn unescape_quotes(source: String, quote_type: QuoteType) -> String {
    match quote_type {
        QuoteType::Double => source.replace(r#"\""#, r#"""#),
        QuoteType::Single => source.replace(r#"\'"#, r#"'"#),
        QuoteType::Raw => source.replace(r#"\`"#, r#"`"#),
    }
}

fn escape_chars(source: String) -> String {
    let reserved_chars = HashSet::from([
        '[', ']', '(', ')', '{', '}', '*', '+', '?', '|', '^', '$', '.', '-', '\\',
    ]);
    let mut escaped_source = String::new();
    for char in source.chars() {
        if reserved_chars.contains(&char) {
            let escaped_char = format!("\\{char}");
            escaped_source.push_str(&escaped_char);
        } else {
            escaped_source.push_str(&String::from(char))
        }
    }
    escaped_source
}

fn escape(source: String, quote_type: QuoteType) -> String {
    escape_chars(unescape_quotes(source, quote_type))
}

fn remove_and_escape(source: &str) -> String {
    let pattern = source[1..source.len() - 1].to_owned();
    let quote = source[0..1].to_owned();
    let quote_type = get_quote_type(&quote);
    escape(pattern, quote_type)
}

fn literal(lex: &mut Lexer<Token>) -> String {
    remove_and_escape(lex.slice())
}

fn raw(lex: &mut Lexer<Token>) -> String {
    let source = lex.slice();
    let pattern = source[1..source.len() - 1].to_owned();
    unescape_quotes(pattern, QuoteType::Raw)
}
