use super::types::pest::Rule;
use crate::errors::CompilerError;
use crate::types::Result;
use once_cell::sync::Lazy;
use pest::iterators::Pair;
use std::collections::HashSet;

pub fn first_inner(pair: Pair<'_, Rule>) -> Result<Pair<'_, Rule>> {
    let last = pair.into_inner().next().ok_or(CompilerError::MissingNode)?;

    Ok(last)
}

pub fn last_inner(pair: Pair<'_, Rule>) -> Result<Pair<'_, Rule>> {
    let last = pair.into_inner().next_back().ok_or(CompilerError::MissingNode)?;

    Ok(last)
}

pub fn first_last_inner_str(pair: Pair<'_, Rule>) -> Result<(&str, &str)> {
    let pairs: Vec<Pair<'_, Rule>> = pair.into_inner().collect();
    Ok((
        pairs.first().ok_or(CompilerError::MissingNode)?.as_str(),
        pairs.last().ok_or(CompilerError::MissingNode)?.as_str(),
    ))
}

pub fn nth_inner(pair: Pair<'_, Rule>, n: usize) -> Option<Pair<'_, Rule>> {
    pair.into_inner().nth(n)
}

pub fn to_char(value: &str) -> Result<char> {
    let char = value.chars().next().ok_or(CompilerError::MissingNode)?;

    Ok(char)
}

pub fn alphabetic_first_char(value: &str) -> Result<bool> {
    Ok(to_char(value)?.is_alphabetic())
}

pub fn unquote_escape_raw(pair: &Pair<'_, Rule>) -> String {
    let pair_str = pair.as_str();
    pair_str[1..pair_str.len() - 1].replace(r"\`", "`")
}

pub fn unquote_escape_literal(pair: &Pair<'_, Rule>) -> String {
    let raw_literal = pair.as_str();
    let quote_type = raw_literal.chars().next().unwrap_or('"');
    let pair_str = escape_chars(raw_literal);
    let literal = pair_str[1..pair_str.len() - 1].to_owned();

    match quote_type {
        '"' => literal.replace(r#"\""#, r#"""#),
        '\'' => literal.replace(r#"\'"#, r#"'"#),
        _ => unreachable!(),
    }
}

static RESERVED_CHARS: Lazy<HashSet<char>> =
    Lazy::new(|| HashSet::from(['[', ']', '(', ')', '{', '}', '*', '+', '?', '|', '^', '$', '.']));

fn escape_chars(source: &str) -> String {
    let mut escaped_source = String::new();
    for char in source.chars() {
        if RESERVED_CHARS.contains(&char) {
            let escaped_char = format!("\\{char}");
            escaped_source.push_str(&escaped_char);
        } else {
            escaped_source.push_str(&String::from(char));
        }
    }
    escaped_source
}
