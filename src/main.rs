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
    let amount: u64 = slice[..slice.len() - 4].parse().ok()?;
    Some(amount.to_string())
}

fn named_capture(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let name = slice[8..slice.len() - 2].to_owned();
    Some(name)
}

fn range_expression(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let range: &str = &slice[..slice.len() - 4];
    let slices = range.split(" to ");

    Some(slices.into_iter().join(","))
}

fn range(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let formatted_slice = slice.replace(" to ", "-");
    Some(format!("[{formatted_slice}]"))
}

fn flags(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let flags = slice[7..slice.len()].to_owned();
    let unique_flags = flags
        .split(",")
        .map(|flag| flag.trim().to_owned())
        .unique()
        .join(",");
    Some(unique_flags)
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(
        "flags: ((has-indices|global|ignore-case|multiline|dot-all|unicode|sticky),( )?)?(has-indices|global|ignore-case|multiline|dot-all|unicode|sticky)",
        flags
    )]
    Flags(String),

    #[regex("\\d+ to \\d+ of ", range_expression)]
    RangeExpression(String),

    #[regex("\\d+ of ", quantifier)]
    QuantifierExpression(String),

    #[regex("([a-zA-Z0-9]|\\\\)+", priority = 300)]
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

    #[token("}")]
    GroupEnd,

    #[token(";")]
    Semicolon,

    #[token("<space>")]
    Whitespace,

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

    let mut regex_flags = None;

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
            Token::Flags(flags) => {
                regex_flags = Some(
                    flags
                        .split(",")
                        .map(|flag| flag_map.get(flag).unwrap())
                        .collect::<Vec<&char>>()
                        .iter()
                        .join(""),
                );
                None
            }
            Token::NamedCapture(name) => {
                group_quantifier = quantifier;
                quantifier = None;
                in_group = true;
                Some(format!("(?<{name}>"))
            }
            Token::Whitespace => Some(String::from("\\s")),
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

    println!(
        "{}",
        format!("/{output}/{}", regex_flags.unwrap_or_default()).blue()
    )
}
