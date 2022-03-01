use super::enums::Symbol;
use super::ident_parser::Rule;
use pest::iterators::Pair;

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

pub fn unquote_escape_literal(pair: Pair<Rule>) -> String {
    let pair_str = pair.as_str();
    pair_str[1..pair_str.len() - 1].to_owned()
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
