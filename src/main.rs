use clap::Parser;
use colored::Colorize;
use itertools::Itertools;
use logos::{Lexer, Logos};
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    query: String,
}

fn quantifier(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let amount: u64 = slice[..slice.len() - 3].parse().ok()?;
    Some(amount.to_string())
}

fn named_capture(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let name = slice[8..slice.len() - 2].to_owned();
    Some(name)
}

fn range_expression(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let range: &str = &slice[..slice.len() - 3];
    let slices = range.split(" to ");

    Some(slices.into_iter().join(","))
}

fn range(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let formatted_slice = slice.replace(" to ", "-");
    Some(format!("[{formatted_slice}]"))
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex("\\d+ to \\d+ of", range_expression)]
    RangeExpression(String),

    #[regex("\\d+ of", quantifier)]
    QuantifierExpression(String),

    #[regex("([a-zA-Z0-9]|\\\\)+", priority = 0)]
    Sequence,

    #[regex("[a-z] to [a-z]", range)]
    LowercaseRange(String),

    #[regex("[A-Z] to [A-Z]", range)]
    UppercaseRange(String),

    #[regex("[0-9] to [0-9]", range)]
    NumericRange(String),

    #[regex("capture [a-z]+ \\{", named_capture)]
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
    SpaceToken,

    #[token("<word>")]
    WordToken,

    #[token("<vertical>")]
    VerticalToken,

    #[token("}")]
    GroupEnd,

    #[token(";")]
    Semicolon,

    #[token("\n")]
    NewLine,

    #[regex("//.*", logos::skip)]
    Comment,

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    Unidentified,
}

fn main() {
    let args = Args::parse();

    let file_path = &args.query;

    let code = read_to_string(file_path).unwrap();

    let mut lex = Token::lexer(&code);

    let mut in_group = false;

    let mut line: u64 = 1;

    let mut quantifier = None;

    let mut group_quantifier = None;

    let mut flag_map: HashMap<&str, char> = HashMap::new();

    let mut output = String::new();

    flag_map.insert("has-indices", 'd');
    flag_map.insert("global", 'g');
    flag_map.insert("ignore-case", 'i');
    flag_map.insert("multiline", 'm');
    flag_map.insert("dot-all", 's');
    flag_map.insert("unicode", 'u');
    flag_map.insert("sticky", 'y');

    while let Some(token) = lex.next() {
        let formatted_token = match token {
            Token::Sequence => {
                let pattern = lex.slice().to_owned();
                if let Some(quantifier) = quantifier.clone() {
                    Some(format!("({pattern}){{{quantifier}}}"))
                } else {
                    Some(pattern)
                }
            }
            Token::LowercaseRange(range)
            | Token::UppercaseRange(range)
            | Token::NumericRange(range) => {
                if let Some(quantifier) = quantifier.clone() {
                    Some(format!("{range}{{{quantifier}}}"))
                } else {
                    Some(range)
                }
            }
            Token::QuantifierExpression(quantity) => {
                quantifier = Some(quantity);
                None
            }
            Token::RangeExpression(range) => {
                quantifier = Some(range);
                None
            }
            Token::NamedCapture(name) => {
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(format!("(?<{name}>"))
            }

            Token::Capture => {
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("("))
            }
            Token::Match => {
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(String::from("(?:"))
            }
            Token::LineStart => Some(String::from("^")),
            Token::LineEnd => Some(String::from("$")),
            Token::Semicolon => {
                quantifier = None;
                None
            }
            Token::GroupEnd => {
                if in_group {
                    in_group = false;
                    if let Some(current_group_quantifier) = group_quantifier {
                        group_quantifier = None;
                        Some(format!("){{{current_group_quantifier}}}"))
                    } else {
                        Some(String::from(")"))
                    }
                } else {
                    None
                }
            }
            Token::SpaceToken => Some(String::from("\\s")),
            Token::NewlineSymbol => Some(String::from("\\n")),
            Token::TabSymbol => Some(String::from("\\t")),
            Token::ReturnSymbol => Some(String::from("\\r")),
            Token::FeedSymbol => Some(String::from("\\f")),
            Token::NullSymbol => Some(String::from("\\0")),
            Token::DigitSymbol => Some(String::from("\\d")),
            Token::WordToken => Some(String::from("\\w")),
            Token::VerticalToken => Some(String::from("\\v")),
            Token::NewLine => {
                line += 1;
                None
            }
            _ => {
                println!(
                    "{} {} {} {}",
                    "Unable to parse".red(),
                    format!("\"{}\"", lex.slice()).blue(),
                    "on line".red(),
                    line.to_string().blue()
                );
                std::process::exit(1);
            }
        };

        if let Some(formatted_token) = formatted_token {
            output.push_str(&formatted_token);
        }
    }

    println!("{}", format!("/{output}/").blue())
}
