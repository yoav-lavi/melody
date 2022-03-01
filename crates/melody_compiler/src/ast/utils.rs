use super::enums::Symbol;
use super::ident_parser::Rule;
use pest::iterators::Pair;
use std::collections::HashSet;

pub fn first_inner(pair: Pair<Rule>) -> Pair<Rule> {
    pair.into_inner().next().unwrap()
}

pub fn last_inner(pair: Pair<Rule>) -> Pair<Rule> {
    pair.into_inner().last().unwrap()
}

pub fn first_last_inner_str(pair: Pair<Rule>) -> (&str, &str) {
    let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();
    (
        pairs.first().unwrap().as_str(),
        pairs.last().unwrap().as_str(),
    )
}

pub fn nth_inner(pair: Pair<Rule>, n: usize) -> Option<Pair<Rule>> {
    pair.into_inner().nth(n)
}

pub fn to_char(value: &str) -> char {
    value.chars().next().unwrap()
}

pub fn alphabetic_first_char(value: &str) -> bool {
    to_char(value).is_alphabetic()
}

pub fn unquote_escape_raw(pair: Pair<Rule>) -> String {
    let pair_str = pair.as_str();
    pair_str[1..pair_str.len() - 1]
        .replace("\\`", "`")
        .to_owned()
}

pub fn unquote_escape_literal(pair: Pair<Rule>) -> String {
    let pair_str = escape_chars(pair.as_str());
    pair_str[1..pair_str.len() - 1]
        .replace("\\\"", "\"")
        .to_owned()
}

fn escape_chars(source: &str) -> String {
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

pub fn symbol_variants(
    negative: bool,
    negative_allowed: bool,
    positive_variant: Symbol,
    negative_variant: Option<Symbol>,
) -> Symbol {
    if negative && !negative_allowed {
        panic!("negative {:?} not allowed", positive_variant)
    }
    if !negative {
        positive_variant
    } else {
        negative_variant.unwrap()
    }
}
