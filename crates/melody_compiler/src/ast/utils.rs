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

pub fn unquote_escape_raw(pair: &Pair<Rule>) -> String {
    let pair_str = pair.as_str();
    pair_str[1..pair_str.len() - 1].replace("\\`", "`")
}

pub fn unquote_escape_literal(pair: &Pair<Rule>) -> String {
    let raw_literal = pair.as_str();
    let quote_type = raw_literal.chars().next().unwrap_or('"');
    let pair_str = escape_chars(raw_literal);
    let literal = pair_str[1..pair_str.len() - 1].to_owned();

    match quote_type {
        '"' => literal.replace(r#"\\""#, r#"""#),
        '\'' => literal.replace(r#"\\'"#, r#"'"#),
        _ => unreachable!(),
    }
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
            escaped_source.push_str(&String::from(char));
        }
    }
    escaped_source
}

/// maps over the items in `iterable` with `closure`,
/// returning a vector of `ReturnItem` or an `Error`
/// if one of the calls to `closure` returns an `Error`
pub fn map_results<Iterable, Item, Closure, ReturnItem, Error>(
    iterable: Iterable,
    closure: Closure,
) -> Result<Vec<ReturnItem>, Error>
where
    Iterable: Iterator<Item = Item>,
    Closure: Fn(Item) -> Result<ReturnItem, Error>,
{
    let mut output = vec![];
    for item in iterable {
        let result = closure(item)?;
        output.push(result);
    }
    Ok(output)
}
