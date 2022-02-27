use super::tokens::Token;
use logos::Lexer;
use std::collections::HashSet;

enum QuoteType {
    Single,
    Double,
    Raw,
}

pub fn quantifier(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let amount: u16 = slice[..slice.len() - 3].parse().ok()?;
    Some(format!("{{{amount}}}"))
}

pub fn named_capture(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    slice[8..slice.len() - 2].to_owned()
}

pub fn range_expression(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    let range: &str = &slice[..slice.len() - 3];
    let formatted_range = range.replace(" to ", ",");
    format!("{{{formatted_range}}}")
}

pub fn open_range_expression(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let range: i32 = slice[5..slice.len() - 3].parse().ok()?;
    let incremented_range = range + 1;

    Some(format!("{{{incremented_range},}}"))
}

pub fn range(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    let formatted_slice = slice.replace(" to ", "-");
    format!("[{formatted_slice}]")
}

pub fn literal(lex: &mut Lexer<Token>) -> String {
    remove_and_escape(lex.slice())
}

pub fn raw(lex: &mut Lexer<Token>) -> String {
    let source = lex.slice();
    let pattern = source[1..source.len() - 1].to_owned();
    unescape_quotes(pattern, QuoteType::Raw)
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
