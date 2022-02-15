use clap::Parser;
use colored::Colorize;
use logos::{Lexer, Logos};
use std::fs::read_to_string;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    source_path: String,
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

fn report_group_end_warning(line: u16) {
    println!(
        "{} {} {} {} {}\n",
        "Warning:".bright_yellow(),
        "Ignoring".bright_yellow(),
        "\"}\"".bright_blue(),
        "on line".bright_yellow(),
        line.to_string().bright_blue()
    );
}

fn report_parse_error(source: &str, line: u16) {
    println!(
        "{} {} {} {} {}\n",
        "Error:".bright_red(),
        "Unable to parse".bright_red(),
        format!("\"{source}\"").bright_blue(),
        "on line".bright_red(),
        line.to_string().bright_blue()
    );
}

fn report_read_file_error(path: &str) {
    println!(
        "{} {} {}",
        "Error:".bright_red(),
        "Unable read file at path".bright_red(),
        format!("\"{path}\"").bright_blue(),
    );
}

fn print_output(output: String) {
    println!("{}", output.bright_blue());
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

fn main() {
    let args = Args::parse();

    let file_path = &args.source_path;

    let raw_source = read_to_string(file_path);

    let source = match raw_source {
        Ok(raw_source) => raw_source,
        Err(_) => {
            report_read_file_error(file_path);
            std::process::exit(1);
        }
    };

    let output = compiler(source);

    print_output(output);
}

fn compiler(source: String) -> String {
    let mut lex = Token::lexer(&source);

    let mut in_group = false;

    let mut line: u16 = 1;

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
            Token::LineStart => Some(String::from("^")),
            Token::LineEnd => Some(String::from("$")),
            Token::SpaceSymbol => Some(String::from("\\s")),
            Token::NewlineSymbol => Some(String::from("\\n")),
            Token::TabSymbol => Some(String::from("\\t")),
            Token::ReturnSymbol => Some(String::from("\\r")),
            Token::FeedSymbol => Some(String::from("\\f")),
            Token::NullSymbol => Some(String::from("\\0")),
            Token::DigitSymbol => Some(String::from("\\d")),
            Token::WordSymbol => Some(String::from("\\w")),
            Token::VerticalSymbol => Some(String::from("\\v")),

            // warning and error related
            Token::NewLine => {
                line += 1;
                None
            }
            _ => {
                report_parse_error(lex.slice(), line);
                std::process::exit(1);
            }
        };

        if let Some(formatted_token) = formatted_token {
            output.push_str(&formatted_token);
        }
    }

    format!("/{output}/")
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
