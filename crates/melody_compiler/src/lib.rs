use logos::{Lexer, Logos};
use std::collections::HashSet;

#[derive(Logos, Debug, PartialEq)]
enum Token {
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

#[derive(Debug, Clone)]
pub struct ParseError {
    /// the unrecognized token responsible for the [ParseError]
    pub token: String,
    /// the line in which an unrecognized token was encountered
    pub line: String,
    /// 0 based index of the line in which an unrecognized token was encountered
    pub line_index: usize,
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

fn handle_quantifier(source: String, quantifier: Option<String>, group: bool) -> Option<String> {
    if let Some(quantifier) = quantifier {
        let formatted_source = if group {
            format!("(?:{source}){quantifier}")
        } else {
            [source, quantifier].join("")
        };
        Some(formatted_source)
    } else {
        Some(source)
    }
}

fn format_regex(regex: &str, flags: Option<String>) -> String {
    format!("/{regex}/{}", flags.unwrap_or_default())
}

fn create_parse_error(lex: &mut Lexer<Token>, line: u16) -> ParseError {
    let line_index = usize::from(line);
    let line_source = lex.source().split('\n').nth(line_index).unwrap();
    ParseError {
        token: lex.slice().to_owned(),
        line: line_source.to_owned(),
        line_index,
    }
}

/**
Compiles Melody source code to a regular expression

see also: [ParseError]

# Example

```rust
use melody_compiler::compiler;

let source = r#"1 to 5 of "A";"#;
let output = compiler(&source);

assert_eq!(output.unwrap(), "/A{1,5}/");
```
*/
pub fn compiler(source: &str) -> Result<String, ParseError> {
    let mut lex = Token::lexer(source);

    let mut in_group = false;

    let mut in_either = false;

    let mut line: u16 = 0;

    let mut quantifier = None;

    let mut group_quantifier = None;

    let mut store: Vec<String> = Vec::new();

    let mut output = String::new();

    while let Some(token) = lex.next() {
        let in_block = in_group || in_either;

        let formatted_token = match token {
            // raw
            Token::LiteralDouble(pattern) | Token::LiteralSingle(pattern) | Token::Raw(pattern) => {
                let group = pattern.chars().count() != 1;
                handle_quantifier(pattern, quantifier.clone(), group)
            }

            // ranges
            Token::LowercaseRange(range)
            | Token::UppercaseRange(range)
            | Token::NumericRange(range) => handle_quantifier(range, quantifier.clone(), false),

            // groups
            Token::Capture => {
                if in_block {
                    return Err(create_parse_error(&mut lex, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("("))
            }
            Token::NamedCapture(name) => {
                if in_block {
                    return Err(create_parse_error(&mut lex, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(format!("(?<{name}>"))
            }
            Token::Match => {
                if in_block {
                    return Err(create_parse_error(&mut lex, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("(?:"))
            }
            Token::Either => {
                if in_block {
                    return Err(create_parse_error(&mut lex, line));
                }
                group_quantifier = quantifier;
                quantifier = None;
                in_either = true;
                None
            }
            Token::BlockEnd => {
                if in_group {
                    in_group = false;
                    let current_group_quantifier = group_quantifier;
                    group_quantifier = None;
                    handle_quantifier(String::from(")"), current_group_quantifier, false)
                } else if in_either {
                    in_either = false;
                    let inner_expressions = store.join("|");
                    store.clear();
                    let current_group_quantifier = group_quantifier;
                    group_quantifier = None;
                    handle_quantifier(
                        format!("(?:{})", inner_expressions),
                        current_group_quantifier,
                        false,
                    )
                } else {
                    None
                }
            }

            // modifiers
            Token::QuantifierExpression(quantity) => {
                quantifier = Some(quantity);
                None
            }
            Token::OpenRangeExpression(range) | Token::RangeExpression(range) => {
                quantifier = Some(range);
                None
            }
            Token::Semicolon => {
                quantifier = None;
                None
            }
            Token::SomeExpression => {
                quantifier = Some(String::from("+"));
                None
            }
            Token::OptionExpression => {
                quantifier = Some(String::from("?"));
                None
            }
            Token::AnyExpression => {
                quantifier = Some(String::from("*"));
                None
            }

            // direct replacements
            Token::StartSymbol => handle_quantifier(String::from("^"), quantifier.clone(), false),
            Token::EndSymbol => handle_quantifier(String::from("$"), quantifier.clone(), false),
            Token::WhitespaceSymbol => handle_quantifier(String::from("\\s"), quantifier.clone(), false),
            Token::NotWhitespaceSymbol => {
                handle_quantifier(String::from("\\S"), quantifier.clone(), false)
            }
            
            Token::SpaceSymbol => handle_quantifier(String::from(" "), quantifier.clone(), false),
            Token::NewlineSymbol => {
                handle_quantifier(String::from("\\n"), quantifier.clone(), false)
            }
            Token::TabSymbol => handle_quantifier(String::from("\\t"), quantifier.clone(), false),
            Token::ReturnSymbol => {
                handle_quantifier(String::from("\\r"), quantifier.clone(), false)
            }
            Token::FeedSymbol => handle_quantifier(String::from("\\f"), quantifier.clone(), false),
            Token::NullSymbol => handle_quantifier(String::from("\\0"), quantifier.clone(), false),
            Token::DigitSymbol => handle_quantifier(String::from("\\d"), quantifier.clone(), false),
            Token::NotDigitSymbol => {
                handle_quantifier(String::from("\\D"), quantifier.clone(), false)
            }
            Token::WordSymbol => handle_quantifier(String::from("\\w"), quantifier.clone(), false),
            Token::NotWordSymbol => {
                handle_quantifier(String::from("\\W"), quantifier.clone(), false)
            }
            Token::AlphabetSymbol => handle_quantifier(String::from("[a-zA-Z]"), quantifier.clone(), false),
            Token::VerticalSymbol => {
                handle_quantifier(String::from("\\v"), quantifier.clone(), false)
            }
            Token::CharSymbol => handle_quantifier(String::from("."), quantifier.clone(), false),

            // warning and error related
            Token::NewLine => {
                line += 1;
                None
            }
            _ => return Err(create_parse_error(&mut lex, line)),
        };

        if let Some(formatted_token) = formatted_token {
            if in_either {
                store.push(formatted_token);
            } else {
                output.push_str(&formatted_token);
            }
        }
    }

    Ok(format_regex(&output, None))
}

#[test]
fn quantifier_test() {
    let output = compiler(
        r#"
  5 of "A";
  "#,
    )
    .unwrap();
    assert_eq!(output, "/A{5}/");
}

#[test]
fn capture_test() {
    let output = compiler(
        r#"
      capture {
        5 of "A";
        0 to 9;
      }
      "#,
    )
    .unwrap();
    assert_eq!(output, "/(A{5}[0-9])/");
}

#[test]
fn named_capture_test() {
    let output = compiler(
        r#"
      capture name {
        5 of "A";
        0 to 9;
      }
      "#,
    )
    .unwrap();
    assert_eq!(output, "/(?<name>A{5}[0-9])/");
}

#[test]
fn number_quantifier_range_test() {
    let output = compiler(
        r#"
      1 to 5 of "A";
      "#,
    )
    .unwrap();
    assert_eq!(output, "/A{1,5}/");
}

#[test]
fn uppercase_range_test() {
    let output = compiler(
        r#"
      A to Z;
      "#,
    )
    .unwrap();
    assert_eq!(output, "/[A-Z]/");
}

#[test]
fn lowercase_range_test() {
    let output = compiler(
        r#"
      a to z;
      "#,
    )
    .unwrap();
    assert_eq!(output, "/[a-z]/");
}

#[test]
fn open_range_expression_test() {
    let output = compiler(
        r#"
      over 4 of "a";
      "#,
    )
    .unwrap();
    assert_eq!(output, "/a{5,}/");
}

#[test]
fn start_end_test() {
    let output = compiler(
        r#"
      <start>;
      "a"
      <end>;
      "#,
    )
    .unwrap();
    assert_eq!(output, "/^a$/");
}

#[test]
fn symbol_test() {
    let output = compiler(
        r#"
      <start>;
      <char>;
      <whitespace>;
      not <whitespace>;
      <newline>;
      <tab>;
      <return>;
      <feed>;
      <null>;
      <digit>;
      not <digit>;
      <word>;
      not <word>;
      <vertical>;
      <alphabet>;
      <space>;
      <end>;
      "#,
    )
    .unwrap();
    assert_eq!(output, r"/^.\s\S\n\t\r\f\0\d\D\w\W\v[a-zA-Z] $/");
}

#[test]
fn single_quote_test() {
    let output = compiler(
        r#"
      'hello';
      "#,
    )
    .unwrap();
    assert_eq!(output, "/hello/");
}

#[test]
fn match_test() {
    let output = compiler(
        r#"
      match {
        5 of "A";
        0 to 9;
      }
      "#,
    )
    .unwrap();
    assert_eq!(output, "/(?:A{5}[0-9])/");
}

#[test]
fn comment_test() {
    let output = compiler(
        r#"
      // a single digit in the range of 0 to 5
      0 to 5;
      "#,
    )
    .unwrap();
    assert_eq!(output, "/[0-5]/");
}

#[test]
fn char_test() {
    let output = compiler(
        r#"
      3 of <char>;
      "#,
    )
    .unwrap();
    assert_eq!(output, "/.{3}/");
}

#[test]
fn some_test() {
    let single_output = compiler(
        r#"
      some of <char>;
      "#,
    )
    .unwrap();
    assert_eq!(single_output, "/.+/");
    let multiple_output = compiler(
        r#"
      some of "ABC";
      "#,
    )
    .unwrap();
    assert_eq!(multiple_output, "/(?:ABC)+/");
}

#[test]
fn option_test() {
    let single_output = compiler(
        r#"
      option of <char>;
      "#,
    )
    .unwrap();
    assert_eq!(single_output, "/.?/");
    let multiple_output = compiler(
        r#"
      option of "ABC";
      "#,
    )
    .unwrap();
    assert_eq!(multiple_output, "/(?:ABC)?/");
}

#[test]
fn either_test() {
    let output = compiler(
        r#"
      either {
        "first";
        "second";
        a to z;
      }
      either {
        "first";
        "second";
      }
      "#,
    )
    .unwrap();
    assert_eq!(output, "/(?:first|second|[a-z])(?:first|second)/");
}

#[test]
fn any_test() {
    let single_output = compiler(
        r#"
      any of <char>;
      "#,
    )
    .unwrap();
    assert_eq!(single_output, "/.*/");
    let multiple_output = compiler(
        r#"
        any of "ABC";
      "#,
    )
    .unwrap();
    assert_eq!(multiple_output, "/(?:ABC)*/");
}

#[test]
fn raw_test() {
    let output = compiler(
        r#"
      5 of `.*`
      "#,
    )
    .unwrap();
    assert_eq!(output, "/(?:.*){5}/");
}
