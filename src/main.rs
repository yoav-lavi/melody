pub mod output;

use clap::Parser;
use logos::{Lexer, Logos};
use output::{
    print_output_pretty, report_group_end_warning, report_parse_error, report_read_file_error,
    report_write_file_error,
};
use std::fs::{read_to_string, write};

use crate::output::print_output;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    path: String,
    #[clap(short, long)]
    file: Option<String>,
    #[clap(short, long)]
    no_color: bool,
}

fn quantifier(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let amount: u16 = slice[..slice.len() - 3].parse().ok()?;
    Some(format!("{{{amount}}}"))
}

fn named_capture(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let name = slice[8..slice.len() - 2].to_owned();
    Some(name)
}

fn range_expression(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let range: &str = &slice[..slice.len() - 3];
    let formatted_range = range.replace(" to ", ",");
    Some(format!("{{{formatted_range}}}"))
}

fn range(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let formatted_slice = slice.replace(" to ", "-");
    Some(format!("[{formatted_slice}]"))
}

fn get_quote_type(quote: &str) -> QuoteType {
    if quote == "\"" {
        QuoteType::Double
    } else {
        QuoteType::Single
    }
}

fn escape_quotes(source: String, quote_type: QuoteType) -> String {
    match quote_type {
        QuoteType::Double => source.replace(r#"\""#, r#"""#),
        QuoteType::Single => source.replace(r#"\'"#, r#"'"#),
    }
}

fn remove_and_escape_quotes(source: &str) -> String {
    let pattern = source[1..source.len() - 1].to_owned();
    let quote = source[0..1].to_owned();
    let quote_type = get_quote_type(&quote);
    escape_quotes(pattern, quote_type)
}

fn raw(lex: &mut Lexer<Token>) -> Option<String> {
    let formatted_raw = remove_and_escape_quotes(lex.slice());
    Some(formatted_raw)
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r#"\d+ to \d+ of"#, range_expression)]
    RangeExpression(String),

    #[regex(r#"\d+ of"#, quantifier)]
    QuantifierExpression(String),

    #[regex(r#""(\\"|[^"\n])*""#, raw)]
    RawDouble(String),

    #[regex(r#"'(\\'|[^'\n])*'"#, raw)]
    RawSingle(String),

    #[regex("[a-z] to [a-z]", range)]
    LowercaseRange(String),

    #[regex("[A-Z] to [A-Z]", range)]
    UppercaseRange(String),

    #[regex("[0-9] to [0-9]", range)]
    NumericRange(String),

    #[regex(r#"capture [a-z]+ \{"#, named_capture)]
    NamedCapture(String),

    #[token("capture {")]
    Capture,

    #[token("match {")]
    Match,

    #[token("start")]
    LineStart,

    #[token("end")]
    LineEnd,

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

    #[token("<space>")]
    SpaceSymbol,

    #[token("<word>")]
    WordSymbol,

    #[token("<vertical>")]
    VerticalSymbol,

    #[token("char")]
    Char,

    #[token("}")]
    GroupEnd,

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
}

fn handle_quantifier(source: String, quantifier: Option<String>, group: bool) -> Option<String> {
    if let Some(quantifier) = quantifier {
        let formatted_source = if group {
            format!("({source}){quantifier}")
        } else {
            [source, quantifier].join("")
        };
        Some(formatted_source)
    } else {
        Some(source)
    }
}

fn format_regex(regex: String, flags: Option<String>) -> String {
    format!("/{regex}/{}", flags.unwrap_or_default())
}

fn main() {
    let args = Args::parse();

    let file_path = &args.path;

    let source = match read_to_string(file_path) {
        Ok(source) => source,
        Err(_) => {
            report_read_file_error(file_path);
            std::process::exit(1);
        }
    };

    let output = compiler(source);

    let output_file_path = &args.file;
    let no_color = args.no_color;

    match output_file_path {
        Some(output_file_path) => {
            let result = write(output_file_path, output);
            if result.is_err() {
                report_write_file_error(output_file_path)
            };
        }
        None => {
            if no_color {
                print_output(output)
            } else {
                print_output_pretty(output)
            }
        }
    }
}

fn compiler(source: String) -> String {
    let mut lex = Token::lexer(&source);

    let mut in_group = false;

    let mut line: u16 = 0;

    let mut quantifier = None;

    let mut group_quantifier = None;

    let mut output = String::new();

    while let Some(token) = lex.next() {
        let formatted_token = match token {
            // raw
            Token::RawDouble(pattern) | Token::RawSingle(pattern) => {
                let group = pattern.chars().count() != 1;
                handle_quantifier(pattern, quantifier.clone(), group)
            }

            // ranges
            Token::LowercaseRange(range)
            | Token::UppercaseRange(range)
            | Token::NumericRange(range) => handle_quantifier(range, quantifier.clone(), false),

            // groups
            Token::Capture => {
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("("))
            }
            Token::NamedCapture(name) => {
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(format!("(?<{name}>"))
            }
            Token::Match => {
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("(?:"))
            }
            Token::GroupEnd => {
                if in_group {
                    in_group = false;
                    let current_group_quantifier = group_quantifier;
                    group_quantifier = None;
                    handle_quantifier(String::from(")"), current_group_quantifier, false)
                } else {
                    report_group_end_warning(line);
                    None
                }
            }

            // modifiers
            Token::QuantifierExpression(quantity) => {
                quantifier = Some(quantity);
                None
            }
            Token::RangeExpression(range) => {
                quantifier = Some(range);
                None
            }
            Token::Semicolon => {
                quantifier = None;
                None
            }

            // direct replacements
            Token::LineStart => handle_quantifier(String::from("^"), quantifier.clone(), false),
            Token::LineEnd => handle_quantifier(String::from("$"), quantifier.clone(), false),
            Token::SpaceSymbol => handle_quantifier(String::from("\\s"), quantifier.clone(), false),
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
            Token::WordSymbol => handle_quantifier(String::from("\\w"), quantifier.clone(), false),
            Token::VerticalSymbol => {
                handle_quantifier(String::from("\\v"), quantifier.clone(), false)
            }
            Token::Char => handle_quantifier(String::from("."), quantifier.clone(), false),

            // warning and error related
            Token::NewLine => {
                line += 1;
                None
            }
            _ => {
                let line_index = usize::from(line);
                let line_source = lex.source().split('\n').nth(line_index).unwrap();
                report_parse_error(lex.slice(), line_source, line + 1);
                std::process::exit(1);
            }
        };

        if let Some(formatted_token) = formatted_token {
            output.push_str(&formatted_token);
        }
    }

    format_regex(output, None)
}

#[test]
fn quantifier_test() {
    let output = compiler(
        r#"
    5 of "A";
    "#
        .to_owned(),
    );
    assert_eq!(output, "/A{5}/");
}

#[test]
fn group_test() {
    let output = compiler(
        r#"
        capture {
          5 of "A";
          0 to 9;
        }
        "#
        .to_owned(),
    );
    assert_eq!(output, "/(A{5}[0-9])/");
}

#[test]
fn comment_test() {
    let output = compiler(
        r#"
        // a single digit in the range of 0 to 5
        0 to 5;
        "#
        .to_owned(),
    );
    assert_eq!(output, "/[0-5]/");
}

#[test]
fn symbol_test() {
    let output = compiler(
        r#"
        <space>;
        <tab>;
        <digit>;
        "#
        .to_owned(),
    );
    assert_eq!(output, r"/\s\t\d/");
}

#[test]
fn char_test() {
    let output = compiler(
        r#"
        3 of char;
        "#
        .to_owned(),
    );
    assert_eq!(output, "/.{3}/");
}
