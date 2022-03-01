use super::consts::NOT;
use super::enums::*;
use super::ident_parser::{IdentParser, Rule};
use super::utils::{
    alphabetic_first_char, first_inner, first_last_inner_str, last_inner, nth_inner,
    symbol_variants, to_char, unquote_escape_literal,
};
use crate::errors::ParseError;
use pest::{iterators::Pair, Parser};

pub fn to_ast(source: &str) -> Result<Vec<Node>, ParseError> {
    let pairs = IdentParser::parse(Rule::root, source).map_err(|error| ParseError {
        message: error.to_string().to_owned(),
    })?;

    let mut ast = Vec::new();

    for pair in pairs {
        for inner in pair.into_inner() {
            ast.push(walk(inner)?);
        }
    }

    Ok(ast)
}

fn walk(pair: Pair<Rule>) -> Result<Node, ParseError> {
    Ok(match pair.as_rule() {
        Rule::raw => Node::Atom(unquote_escape_literal(pair)),
        Rule::literal => Node::Atom(unquote_escape_literal(pair)),

        Rule::symbol => {
            let (negative, ident) = first_last_inner_str(pair);

            let negative = negative == NOT;

            match ident {
                "space" => Node::Symbol(symbol_variants(negative, false, Symbol::Space, None)),
                "newline" => Node::Symbol(symbol_variants(negative, false, Symbol::Newline, None)),
                "vertical" => {
                    Node::Symbol(symbol_variants(negative, false, Symbol::Vertical, None))
                }
                "word" => Node::Symbol(symbol_variants(
                    negative,
                    true,
                    Symbol::Word,
                    Some(Symbol::NotWord),
                )),
                "digit" => Node::Symbol(symbol_variants(
                    negative,
                    true,
                    Symbol::Digit,
                    Some(Symbol::NotDigit),
                )),
                "whitespace" => Node::Symbol(symbol_variants(
                    negative,
                    true,
                    Symbol::Whitespace,
                    Some(Symbol::NotWhitespace),
                )),
                "return" => Node::Symbol(symbol_variants(negative, false, Symbol::Return, None)),
                "tab" => Node::Symbol(symbol_variants(negative, false, Symbol::Tab, None)),
                "null" => Node::Symbol(symbol_variants(negative, false, Symbol::Null, None)),
                "alphabet" => {
                    Node::Symbol(symbol_variants(negative, false, Symbol::Alphabet, None))
                }
                "feed" => Node::Symbol(symbol_variants(negative, false, Symbol::Feed, None)),
                "char" => Node::Symbol(symbol_variants(negative, false, Symbol::Char, None)),

                "start" => Node::SpecialSymbol(SpecialSymbol::Start),
                "end" => Node::SpecialSymbol(SpecialSymbol::End),

                _ => unreachable!(),
            }
        }
        Rule::atom => Node::Atom(pair.as_str().to_owned()),

        Rule::range => {
            let (start, end) = first_last_inner_str(pair);
            if alphabetic_first_char(start) {
                Node::Range(Range::AsciiRange(AsciiRange {
                    start: to_char(start),
                    end: to_char(end),
                }))
            } else {
                Node::Range(Range::NumericRange(NumericRange {
                    start: to_char(start),
                    end: to_char(end),
                }))
            }
        }

        Rule::quantifier => {
            let quantity = first_inner(pair.clone());
            let kind = first_inner(quantity);
            let expression = walk(last_inner(pair))?;

            let expression = match expression {
                Node::Group(group) => Expression::Group(group),
                Node::Atom(atom) => Expression::Atom(atom),
                Node::Range(range) => Expression::Range(range),
                Node::Symbol(symbol) => Expression::Symbol(symbol),

                // unexpected nodes
                Node::SpecialSymbol(_) => {
                    return Err(ParseError {
                        message: "unexpected special symbol in expression".to_owned(),
                    })
                }
                Node::Quantifier(_) => {
                    return Err(ParseError {
                        message: "unexpected quantifier in expression".to_owned(),
                    })
                }
                Node::Assertion(_) => {
                    return Err(ParseError {
                        message: "unexpected assertion in expression".to_owned(),
                    })
                }

                Node::EndOfInput => {
                    return Err(ParseError {
                        message: "unexpected end of input".to_owned(),
                    })
                }
            };

            match kind.as_rule() {
                Rule::amount => Node::Quantifier(Quantifier {
                    kind: QuantifierKind::Amount(kind.as_str().to_owned()),
                    expression: Box::new(expression),
                }),
                Rule::over => Node::Quantifier(Quantifier {
                    kind: QuantifierKind::Over(last_inner(kind).as_str().to_owned()),
                    expression: Box::new(expression),
                }),
                Rule::option => Node::Quantifier(Quantifier {
                    kind: QuantifierKind::Option,
                    expression: Box::new(expression),
                }),
                Rule::any => Node::Quantifier(Quantifier {
                    kind: QuantifierKind::Any,
                    expression: Box::new(expression),
                }),
                Rule::some => Node::Quantifier(Quantifier {
                    kind: QuantifierKind::Some,
                    expression: Box::new(expression),
                }),

                Rule::quantifier_range => {
                    let (start, end) = first_last_inner_str(kind);
                    Node::Quantifier(Quantifier {
                        kind: QuantifierKind::Range {
                            start: start.to_owned(),
                            end: end.to_owned(),
                        },
                        expression: Box::new(expression),
                    })
                }

                _ => unreachable!(),
            }
        }

        Rule::group => {
            let declaration = first_inner(pair.clone());

            let kind = first_inner(declaration.clone()).as_str();

            let kind = match kind {
                "either" => GroupKind::Either,
                "capture" => GroupKind::Capture,
                "match" => GroupKind::Match,

                _ => unreachable!(),
            };

            let ident = nth_inner(declaration, 1).map(|ident| ident.as_str().trim().to_owned());

            if ident.is_some() && kind != GroupKind::Capture {
                panic!("unexpected identifier")
            }
            let block = last_inner(pair);

            let statements = statements_from_block(block)?;

            Node::Group(Group {
                ident,
                kind,
                statements,
            })
        }

        Rule::assertion => {
            let assertion_declaration = first_inner(pair.clone());

            let (negative, kind) = first_last_inner_str(assertion_declaration);

            let negative = negative == NOT;

            let kind = match kind {
                "ahead" => AssertionKind::Ahead,
                "behind" => AssertionKind::Behind,
                _ => AssertionKind::Ahead,
            };

            let block = last_inner(pair);

            let statements = statements_from_block(block)?;

            Node::Assertion(Assertion {
                kind,
                statements,
                negative,
            })
        }
        Rule::EOI => Node::EndOfInput,

        _ => unreachable!(),
    })
}

fn statements_from_block(pair: Pair<Rule>) -> Result<Vec<Node>, ParseError> {
    let mut nodes = vec![];
    for statement in pair.into_inner() {
        let result = walk(statement)?;
        nodes.push(result);
    }
    Ok(nodes)
}
