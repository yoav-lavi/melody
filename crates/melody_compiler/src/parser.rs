#![allow(dead_code)]
use crate::ast::*;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "melody.pest"]
pub struct MelodyParser;

pub fn parse(s: &str) -> Result<Vec<Expr>, Error<Rule>> {
    let pairs = MelodyParser::parse(Rule::program, s)?;
    pairs
        .into_iter()
        .filter_map(|pair| match pair.as_rule() {
            Rule::expr => Some(parse_expr(pair.into_inner().next().unwrap())),
            _ => None,
        })
        .collect()
}

pub fn parse_expr(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    match pair.as_rule() {
        Rule::expr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::keyword => parse_keyword(pair),
        Rule::digit_range => parse_digit_range(pair),
        Rule::ascii_range => parse_char_range(pair),
        Rule::string => parse_string(pair),
        Rule::quantifier_expr => parse_quantifier_expr(pair),
        Rule::capture_expr => parse_capture_expr(pair),
        Rule::match_expr => parse_match_expr(pair),
        _ => panic!("unexpected rule {}", pair),
    }
}

pub fn parse_keyword(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    Ok(Expr::Keyword(pair.as_str().into()))
}

pub fn parse_digit_range(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    let mut inner = pair.into_inner();
    let start: u8 = inner.next().unwrap().as_str().parse().unwrap();
    let end: u8 = inner.next().unwrap().as_str().parse().unwrap();
    Ok(Expr::DigitRange { start, end })
}

pub fn parse_char_range(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    let mut inner = pair.into_inner();
    let start: char = inner.next().unwrap().as_str().parse().unwrap();
    let end: char = inner.next().unwrap().as_str().parse().unwrap();
    Ok(Expr::CharRange { start, end })
}

pub fn parse_string(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    Ok(Expr::Literal(pair.into_inner().next().unwrap().as_str()))
}

pub fn parse_quantifier_expr(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    let mut inner = pair.into_inner();
    let quantifier_range = parse_range(inner.next().unwrap())?;
    let expr = parse_expr(inner.next().unwrap())?;
    Ok(Expr::Quantifier {
        range: quantifier_range,
        expr: Box::new(expr),
    })
}

pub fn parse_range(pair: Pair<Rule>) -> Result<Range, Error<Rule>> {
    let range = match pair.as_rule() {
        Rule::exact_range => {
            let mut inner = pair.into_inner();
            let exact: u64 = inner.next().unwrap().as_str().parse().unwrap();
            Range::Exact(exact)
        }
        Rule::int_range => {
            let mut inner = pair.into_inner();
            let from: u64 = inner.next().unwrap().as_str().parse().unwrap();
            let to: u64 = inner.next().unwrap().as_str().parse().unwrap();
            Range::Numeric(from..to + 1)
        }

        Rule::over_range => {
            let mut inner = pair.into_inner();
            let from: u64 = inner.next().unwrap().as_str().parse().unwrap();
            Range::Over(from + 1..)
        }
        Rule::upto_range => {
            let mut inner = pair.into_inner();
            let to: u64 = inner.next().unwrap().as_str().parse().unwrap();
            Range::Upto(..to + 1)
        }
        Rule::range_keywords => match pair.as_str() {
            "some" => Range::Over(1..),
            "any" => Range::Over(0..),
            _ => panic!("unkown range keyword {}", pair),
        },
        _ => unreachable!(),
    };

    Ok(range)
}

pub fn parse_capture_expr(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    let mut inner = pair.into_inner();

    let next = inner.next().unwrap();
    let (capture_name, capture_exprs): (Option<&str>, Result<Vec<Expr>, Error<Rule>>) =
        match next.as_rule() {
            Rule::name => {
                let exprs = inner.next().unwrap().into_inner();
                (Some(next.as_str()), exprs.map(parse_expr).collect())
            }
            Rule::blocked_expr => (None, next.into_inner().map(parse_expr).collect()),
            _ => unreachable!(),
        };

    Ok(Expr::Capture {
        capture_name,
        capture_exprs: capture_exprs?.into(),
    })
}

pub fn parse_match_expr(pair: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    let mut inner = pair.into_inner();

    let next = inner.next().unwrap();
    let (match_name, match_exprs): (Option<&str>, Result<Vec<Expr>, Error<Rule>>) =
        match next.as_rule() {
            Rule::name => {
                let exprs = inner.next().unwrap().into_inner();
                (Some(next.as_str()), exprs.map(parse_expr).collect())
            }
            Rule::blocked_expr => (None, next.into_inner().map(parse_expr).collect()),
            _ => unreachable!(),
        };

    Ok(Expr::Match {
        match_name,
        match_exprs: match_exprs?.into(),
    })
}

#[test]
fn atom_keyword() {
    let ast = parse("<null>;").unwrap();
    assert_eq!(ast, &[Expr::Keyword(Keyword::NullSymbol)]);
}

#[test]
fn literal() {
    let ast = parse(
        r#"
        "lit str";
        <newline>;
        "#,
    )
    .unwrap();
    assert_eq!(
        ast,
        &[Expr::Literal("lit str"), Expr::Keyword(Keyword::NewLine)]
    );
}

#[test]
fn exact_range() {
    let ast = parse(
        r#"
        10 of "A";
        "#,
    )
    .unwrap();
    assert_eq!(
        ast,
        &[Expr::Quantifier {
            range: Range::Exact(10),
            expr: Box::new(Expr::Literal("A"))
        }]
    );
}

#[test]
fn int_range() {
    let ast = parse(
        r#"
        1 to 10 of "A";
        "#,
    )
    .unwrap();
    assert_eq!(
        ast,
        &[Expr::Quantifier {
            range: Range::Numeric(1..11),
            expr: Box::new(Expr::Literal("A"))
        }]
    );
}

#[test]
fn over_range() {
    let ast = parse(
        r#"
        over 10 of "A";
        "#,
    )
    .unwrap();
    assert_eq!(
        ast,
        &[Expr::Quantifier {
            range: Range::Over(11..),
            expr: Box::new(Expr::Literal("A"))
        }]
    );
}

#[test]
fn upto_range() {
    let ast = parse(
        r#"
        upto 10 of "A";
        "#,
    )
    .unwrap();
    assert_eq!(
        ast,
        &[Expr::Quantifier {
            range: Range::Upto(..11),
            expr: Box::new(Expr::Literal("A"))
        }]
    );
}

#[test]
fn some_range() {
    let ast = parse(
        r#"
        some of "A";
        "#,
    )
    .unwrap();
    assert_eq!(
        ast,
        &[Expr::Quantifier {
            range: Range::Over(1..),
            expr: Box::new(Expr::Literal("A"))
        }]
    );
}

#[test]
fn any_range() {
    let ast = parse(
        r#"
        any of "A";
        "#,
    )
    .unwrap();
    assert_eq!(
        ast,
        &[Expr::Quantifier {
            range: Range::Over(0..),
            expr: Box::new(Expr::Literal("A"))
        }]
    );
}

#[test]
fn capture() {
    let ast = parse(
        r#"
        capture {
        "A";
      }
        "#,
    )
    .unwrap();
    assert_eq!(
        ast,
        &[Expr::Capture {
            capture_name: None,
            capture_exprs: MultipleExprs(vec![Expr::Literal("A")])
        }]
    );
}
