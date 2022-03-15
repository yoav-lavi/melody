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
    let node = match pair.as_rule() {
        Rule::raw => Node::Atom(unquote_escape_raw(&pair)),
        Rule::literal => Node::Atom(unquote_escape_literal(&pair)),
        Rule::symbol => symbol(pair)?,
        Rule::range => range(pair)?,
        Rule::quantifier => quantifier(pair, variables)?,
        Rule::group => group(pair, variables)?,
        Rule::assertion => assertion(pair, variables)?,
        Rule::negative_char_class => negative_char_class(&pair)?,
        Rule::variable_invocation => variable_invocation(&pair, variables)?,
        Rule::variable_declaration => variable_declaration(pair, variables)?,
        Rule::EOI => Node::Empty,
        _ => return Err(ErrorMessage::UnrecognizedSyntax.into()),
    };

    Ok(node)
}

fn symbol(pair: Pair<Rule>) -> Result<Node, ParseError> {
    let (negative, ident) = first_last_inner_str(pair)?;

    let negative = negative == NOT;

    if negative {
        match ident {
            "start" => return Err(ErrorMessage::NegativeStartNotAllowed.into()),
            "end" => return Err(ErrorMessage::NegativeEndNotAllowed.into()),
            _ => {}
        }
    }

    let symbol_node = match ident {
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

        _ => return Err(ErrorMessage::UnrecognizedSymbol.into()),
    };

    Ok(symbol_node)
}

fn range(pair: Pair<Rule>) -> Result<Node, ParseError> {
    let (first, end) = first_last_inner_str(pair.clone())?;
    let negative = first == NOT;
    let start = if negative {
        nth_inner(pair, 1)
            .ok_or_else(|| ParseError::from(ErrorMessage::MissingNode))?
            .as_str()
    } else {
        first
    };
    let range_node = if alphabetic_first_char(start)? {
        Node::Range(Range::AsciiRange(AsciiRange {
            negative,
            start: to_char(start)?,
            end: to_char(end)?,
        }))
    } else {
        Node::Range(Range::NumericRange(NumericRange {
            negative,
            start: to_char(start)?,
            end: to_char(end)?,
        }))
    };

    Ok(range_node)
}

fn quantifier(
    pair: Pair<Rule>,
    variables: &mut HashMap<String, Vec<Node>>,
) -> Result<Node, ParseError> {
    let quantity = first_inner(pair.clone())?;
    let kind = first_inner(quantity.clone())?;
    let expression = create_ast_node(last_inner(pair)?, variables)?;

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
        Node::Quantifier(_) => return Err(ErrorMessage::UnexpectedQuantifierInQuantifier.into()),
        Node::Assertion(_) => return Err(ErrorMessage::UnexpectedAssertionInQuantifier.into()),
        Node::Empty => return Err(ErrorMessage::UnexpectedEmptyNodeInQuantifier.into()),
        Node::VariableInvocation(_) => {
            return Err(ErrorMessage::UnexpectedVariableInvocationInQuantifier.into())
        }
    };

    let lazy = quantity.as_str().starts_with(LAZY);

    let quantifier_node = match kind.as_rule() {
        Rule::amount => Node::Quantifier(Quantifier {
            kind: QuantifierKind::Amount(kind.as_str().to_owned()),
            lazy,
            expression: Box::new(expression),
        }),
        Rule::over => {
            let raw_amount = last_inner(kind)?.as_str().to_owned();
            let amount = raw_amount
                .parse::<usize>()
                .map_err(|_| ParseError::from(ErrorMessage::CouldNotParseAnAmount))?
                .checked_add(1)
                .ok_or_else(|| ParseError::from(ErrorMessage::CouldNotParseAnAmount))?;
            Node::Quantifier(Quantifier {
                kind: QuantifierKind::Over(amount),
                lazy,
                expression: Box::new(expression),
            })
        }
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
            let (start, end) = first_last_inner_str(kind)?;
            Node::Quantifier(Quantifier {
                kind: QuantifierKind::Range {
                    start: start.to_owned(),
                    end: end.to_owned(),
                },
                lazy,
                expression: Box::new(expression),
            })
        }

        _ => return Err(ErrorMessage::UnrecognizedSyntax.into()),
    };

    Ok(quantifier_node)
}

fn group(pair: Pair<Rule>, variables: &mut HashMap<String, Vec<Node>>) -> Result<Node, ParseError> {
    let declaration = first_inner(pair.clone())?;

    let kind = first_inner(declaration.clone())?.as_str();

    let kind = match kind {
        "either" => GroupKind::Either,
        "capture" => GroupKind::Capture,
        "match" => GroupKind::Match,

        _ => return Err(ErrorMessage::UnrecognizedGroup.into()),
    };

    let ident = nth_inner(declaration, 1).map(|ident| ident.as_str().trim().to_owned());

    if ident.is_some() && kind != GroupKind::Capture {
        return Err(ErrorMessage::UnexpectedIdentifierForNonCaptureGroup.into());
    }

    let block = last_inner(pair)?;

    let statements = map_results(block.into_inner(), &mut |statement| {
        create_ast_node(statement, variables)
    })?;

    let group_node = Node::Group(Group {
        ident,
        kind,
        statements,
    });

    Ok(group_node)
}

fn assertion(
    pair: Pair<Rule>,
    variables: &mut HashMap<String, Vec<Node>>,
) -> Result<Node, ParseError> {
    let assertion_declaration = first_inner(pair.clone())?;

    let (negative, kind) = first_last_inner_str(assertion_declaration)?;

    let negative = negative == NOT;

    let kind = match kind {
        "ahead" => AssertionKind::Ahead,
        "behind" => AssertionKind::Behind,
        _ => return Err(ErrorMessage::UnrecognizedAssertion.into()),
    };

    let block = last_inner(pair)?;

    let statements = map_results(block.into_inner(), &mut |statement| {
        create_ast_node(statement, variables)
    })?;

    let assertion_node = Node::Assertion(Assertion {
        kind,
        statements,
        negative,
    });

    Ok(assertion_node)
}

fn negative_char_class(pair: &Pair<Rule>) -> Result<Node, ParseError> {
    let class = last_inner(pair.clone())?;
    let negative_char_class_node = Node::NegativeCharClass(class.as_str().to_owned());
    Ok(negative_char_class_node)
}

fn variable_invocation(
    pair: &Pair<Rule>,
    variables: &mut HashMap<String, Vec<Node>>,
) -> Result<Node, ParseError> {
    let identifier = last_inner(pair.clone())?;
    let statements = match variables.get(identifier.as_str()) {
        Some(statements) => statements.clone(),
        None => return Err(ErrorMessage::UninitializedVariable.into()),
    };
    let variable_invocation_node = Node::VariableInvocation(VariableInvocation { statements });
    Ok(variable_invocation_node)
}

fn variable_declaration(
    pair: Pair<Rule>,
    variables: &mut HashMap<String, Vec<Node>>,
) -> Result<Node, ParseError> {
    let identifier = first_inner(pair.clone())?;
    let statements = last_inner(pair)?;
    variables.insert(
        identifier.as_str().trim().to_owned(),
        map_results(statements.into_inner(), &mut |statement| {
            create_ast_node(statement, &mut variables.clone())
        })?,
    );
    Ok(Node::Empty)
}
