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

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    Unidentified,
}

fn main() {
    let args = Args::parse();

    let file_path = &args.query;

    let code = read_to_string(file_path).unwrap();

    let mut lex = Token::lexer(&code);

    let mut output = String::new();

    let mut in_group: bool = false;

    let mut line: u64 = 1;

    let mut quantifier = String::new();

    let mut group_quantifier = String::new();

    let mut regex_flags = String::new();

    let mut flag_map: HashMap<&str, char> = HashMap::new();

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
                if !quantifier.is_empty() {
                    format!("({pattern}){{{quantifier}}}")
                } else {
                    pattern
                }
            }
            Token::LowercaseRange(range)
            | Token::UppercaseRange(range)
            | Token::NumericRange(range) => {
                if !quantifier.is_empty() {
                    format!("{range}{{{quantifier}}}")
                } else {
                    range
                }
            }
            Token::QuantifierExpression(quantity) => {
                quantifier = quantity;
                String::new()
            }
            Token::RangeExpression(range) => {
                quantifier = range;
                String::new()
            }
            Token::Flags(flags) => {
                regex_flags = flags
                    .split(",")
                    .map(|flag| flag_map.get(flag).unwrap())
                    .collect::<Vec<&char>>()
                    .iter()
                    .join("")
                    .to_owned();
                String::new()
            }
            Token::NamedCapture(name) => {
                group_quantifier = quantifier;
                quantifier = String::new();
                in_group = true;
                format!("(?<{name}>")
            }
            Token::Whitespace => String::from("\\s"),
            Token::Capture => {
                group_quantifier = quantifier;
                quantifier = "".to_owned();
                in_group = true;
                String::from("(")
            }
            Token::Match => {
                group_quantifier = quantifier;
                quantifier = String::new();
                in_group = true;
                String::from("(?:")
            }
            Token::Semicolon => {
                quantifier = String::new();
                String::new()
            }
            Token::GroupEnd => {
                if in_group {
                    in_group = false;
                    if !group_quantifier.is_empty() {
                        let previous_group_quantifier = group_quantifier;
                        print!("");
                        group_quantifier = String::new();
                        format!("){{{previous_group_quantifier}}}")
                    } else {
                        String::from(")")
                    }
                } else {
                    String::new()
                }
            }
            Token::NewLine => {
                line += 1;
                String::new()
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

        output.push_str(&formatted_token);
    }
    println!("{}", format!("/{output}/{regex_flags}").blue())
}
