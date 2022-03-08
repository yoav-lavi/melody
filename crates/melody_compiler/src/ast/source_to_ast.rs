use super::consts::{LAZY, NOT};
use super::enums::*;
use super::error_messages::ErrorMessage;
use super::ident_parser::{IdentParser, Rule};
use super::utils::{
    alphabetic_first_char, first_inner, first_last_inner_str, last_inner, map_results, nth_inner,
    to_char, unquote_escape_literal, unquote_escape_raw,
};
use crate::errors::ParseError;
use pest::{iterators::Pair, Parser};
use std::collections::HashMap;

pub fn to_ast(source: &str) -> Result<Vec<Node>, ParseError> {
    let mut variables: HashMap<String, Vec<Node>> = HashMap::new();

    let mut pairs = IdentParser::parse(Rule::root, source).map_err(|error| ParseError {
        message: error.to_string(),
    })?;

    let mut ast = Vec::new();

    let root = pairs
        .next()
        .ok_or_else(|| ParseError::from(ErrorMessage::MissingRootNode))?;

    for statement in root.into_inner() {
        let node = create_ast_node(statement, &mut variables)?;
        ast.push(node);
    }

    Ok(ast)
}

fn create_ast_node(
    pair: Pair<Rule>,
    variables: &mut HashMap<String, Vec<Node>>,
) -> Result<Node, ParseError> {
    Ok(match pair.as_rule() {
        Rule::raw => Node::Atom(unquote_escape_raw(&pair)),
        Rule::literal => Node::Atom(unquote_escape_literal(&pair)),

        Rule::symbol => {
            let (negative, ident) = first_last_inner_str(pair);

            let negative = negative == NOT;

            if negative {
                match ident {
                    "start" => return Err(ErrorMessage::NegativeStartNotAllowed.into()),
                    "end" => return Err(ErrorMessage::NegativeEndNotAllowed.into()),
                    _ => {}
                }
            }

            match ident {
                "space" => Node::Symbol(Symbol {
                    kind: SymbolKind::Space,
                    negative,
                }),
                "newline" => Node::Symbol(Symbol {
                    kind: SymbolKind::Newline,
                    negative,
                }),
                "vertical" => Node::Symbol(Symbol {
                    kind: SymbolKind::Vertical,
                    negative,
                }),
                "word" => Node::Symbol(Symbol {
                    kind: SymbolKind::Word,
                    negative,
                }),
                "digit" => Node::Symbol(Symbol {
                    kind: SymbolKind::Digit,
                    negative,
                }),
                "whitespace" => Node::Symbol(Symbol {
                    kind: SymbolKind::Whitespace,
                    negative,
                }),
                "boundary" => Node::Symbol(Symbol {
                    kind: SymbolKind::Boundary,
                    negative,
                }),
                "alphabetic" => Node::Symbol(Symbol {
                    kind: SymbolKind::Alphabetic,
                    negative,
                }),
                "alphanumeric" => Node::Symbol(Symbol {
                    kind: SymbolKind::Alphanumeric,
                    negative,
                }),
                "return" => Node::Symbol(Symbol {
                    kind: SymbolKind::Return,
                    negative,
                }),
                "tab" => Node::Symbol(Symbol {
                    kind: SymbolKind::Tab,
                    negative,
                }),
                "null" => Node::Symbol(Symbol {
                    kind: SymbolKind::Null,
                    negative,
                }),
                "feed" => Node::Symbol(Symbol {
                    kind: SymbolKind::Feed,
                    negative,
                }),
                "char" => Node::Symbol(Symbol {
                    kind: SymbolKind::Char,
                    negative,
                }),
                "backspace" => Node::Symbol(Symbol {
                    kind: SymbolKind::Backspace,
                    negative,
                }),

                "start" => Node::SpecialSymbol(SpecialSymbol::Start),
                "end" => Node::SpecialSymbol(SpecialSymbol::End),

                _ => unreachable!(),
            }
        }

        Rule::range => {
            let (first, end) = first_last_inner_str(pair.clone());
            let negative = first == NOT;
            let start = if negative {
                nth_inner(pair, 1).unwrap().as_str()
            } else {
                first
            };
            if alphabetic_first_char(start) {
                Node::Range(Range::AsciiRange(AsciiRange {
                    negative,
                    start: to_char(start),
                    end: to_char(end),
                }))
            } else {
                Node::Range(Range::NumericRange(NumericRange {
                    negative,
                    start: to_char(start),
                    end: to_char(end),
                }))
            }
        }

        Rule::quantifier => {
            let quantity = first_inner(pair.clone());
            let kind = first_inner(quantity.clone());
            let expression = create_ast_node(last_inner(pair), variables)?;

            let expression = match expression {
                Node::Group(group) => Expression::Group(group),
                Node::Atom(atom) => Expression::Atom(atom),
                Node::Range(range) => Expression::Range(range),
                Node::Symbol(symbol) => Expression::Symbol(symbol),
                Node::NegativeCharClass(class) => Expression::NegativeCharClass(class),

                // unexpected nodes
                Node::SpecialSymbol(_) => {
                    return Err(ErrorMessage::UnexpectedSpecialSymbolInQuantifier.into())
                }
                Node::Quantifier(_) => {
                    return Err(ErrorMessage::UnexpectedQuantifierInQuantifier.into())
                }
                Node::Assertion(_) => {
                    return Err(ErrorMessage::UnexpectedAssertionInQuantifier.into())
                }
                Node::Empty => return Err(ErrorMessage::UnexpectedEmptyNodeInQuantifier.into()),
                Node::VariableInvocation(_) => {
                    return Err(ErrorMessage::UnexpectedVariableInvocationInQuantifier.into())
                }
            };

            let lazy = quantity.as_str().starts_with(LAZY);

            match kind.as_rule() {
                Rule::amount => Node::Quantifier(Quantifier {
                    kind: QuantifierKind::Amount(kind.as_str().to_owned()),
                    lazy,
                    expression: Box::new(expression),
                }),
                Rule::over => Node::Quantifier(Quantifier {
                    kind: QuantifierKind::Over(last_inner(kind).as_str().to_owned()),
                    lazy,
                    expression: Box::new(expression),
                }),
                Rule::option => Node::Quantifier(Quantifier {
                    kind: QuantifierKind::Option,
                    lazy,
                    expression: Box::new(expression),
                }),
                Rule::any => Node::Quantifier(Quantifier {
                    kind: QuantifierKind::Any,
                    lazy,
                    expression: Box::new(expression),
                }),
                Rule::some => Node::Quantifier(Quantifier {
                    kind: QuantifierKind::Some,
                    lazy,
                    expression: Box::new(expression),
                }),

                Rule::quantifier_range => {
                    let (start, end) = first_last_inner_str(kind);
                    Node::Quantifier(Quantifier {
                        kind: QuantifierKind::Range {
                            start: start.to_owned(),
                            end: end.to_owned(),
                        },
                        lazy,
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
                return Err(ErrorMessage::UnexpectedIdentifierForNonCaptureGroup.into());
            }
            let block = last_inner(pair);

            let statements = map_results(block.into_inner(), &mut |statement| {
                create_ast_node(statement, variables)
            })?;

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
                _ => unreachable!(),
            };

            let block = last_inner(pair);

            let statements = map_results(block.into_inner(), &mut |statement| {
                create_ast_node(statement, variables)
            })?;

            Node::Assertion(Assertion {
                kind,
                statements,
                negative,
            })
        }
        Rule::negative_char_class => {
            let class = last_inner(pair.clone());
            Node::NegativeCharClass(class.as_str().to_owned())
        }
        Rule::variable_invocation => {
            let identifier = last_inner(pair.clone());
            let statements = variables.get(identifier.as_str());
            if statements.is_none() {
                return Err(ErrorMessage::UninitializedVariable.into());
            }
            Node::VariableInvocation(VariableInvocation {
                statements: statements.unwrap().clone(),
            })
        }
        Rule::variable_declaration => {
            let identifier = first_inner(pair.clone());
            let statements = last_inner(pair);
            variables.insert(
                identifier.as_str().to_owned(),
                map_results(statements.into_inner(), &mut |statement| {
                    create_ast_node(statement, &mut variables.clone())
                })?,
            );
            Node::Empty
        }
        Rule::EOI => Node::Empty,

        _ => unreachable!(),
    })
}
