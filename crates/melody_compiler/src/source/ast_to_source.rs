use super::utils::wrap_quantified;
use crate::ast::enums::{
    AssertionKind, Expression, GroupKind, Node, QuantifierKind, Range, SpecialSymbol, Symbol,
};

pub fn to_source(ast: &[Node]) -> String {
    let mut output = String::new();
    for node in ast {
        output.push_str(node_to_source(node).as_str());
    }
    output
}

fn node_to_source(node: &Node) -> String {
    let mut output = String::new();

    match node {
        Node::Group(group) => match group.kind {
            GroupKind::Match => {
                let body_source = to_source(&group.statements);
                output.push_str(format!("(?:{body_source})").as_str())
            }
            GroupKind::Capture => {
                let body_source = to_source(&group.statements);
                if group.ident.is_some() {
                    output.push_str(
                        format!("(?<{}>{body_source})", group.ident.as_ref().unwrap()).as_str(),
                    )
                } else {
                    output.push_str(format!("({body_source})").as_str())
                }
            }
            GroupKind::Either => {
                let body_source = group
                    .statements
                    .iter()
                    .map(node_to_source)
                    .collect::<Vec<String>>()
                    .join("|");
                output.push_str(format!("(?:{body_source})").as_str())
            }
        },

        Node::Quantifier(quantifier) => match &quantifier.kind {
            QuantifierKind::Range { start, end } => output.push_str(
                format!(
                    "{}{{{start},{end}}}",
                    wrap_quantified(expression_to_source(&quantifier.expression))
                )
                .as_str(),
            ),
            QuantifierKind::Some => output.push_str(
                format!(
                    "{}+",
                    wrap_quantified(expression_to_source(&quantifier.expression))
                )
                .as_str(),
            ),
            QuantifierKind::Any => output.push_str(
                format!(
                    "{}*",
                    wrap_quantified(expression_to_source(&quantifier.expression))
                )
                .as_str(),
            ),
            QuantifierKind::Over(amount) => output.push_str(
                format!(
                    "{}{{{},}}",
                    wrap_quantified(expression_to_source(&quantifier.expression)),
                    amount.parse::<usize>().unwrap() + 1
                )
                .as_str(),
            ),
            QuantifierKind::Option => output.push_str(
                format!(
                    "{}?",
                    wrap_quantified(expression_to_source(&quantifier.expression))
                )
                .as_str(),
            ),
            QuantifierKind::Amount(amount) => output.push_str(
                format!(
                    "{}{{{amount}}}",
                    wrap_quantified(expression_to_source(&quantifier.expression)),
                )
                .as_str(),
            ),
        },
        Node::Atom(atom) => output.push_str(atom.as_str()),

        Node::Symbol(symbol) => {
            let transformed_symbol = match symbol {
                Symbol::Space => " ",
                Symbol::Newline => "\\n",
                Symbol::Vertical => "\\v",
                Symbol::Return => "\\r",
                Symbol::Tab => "\\t",
                Symbol::Null => "\\0",
                Symbol::Whitespace => "\\s",
                Symbol::NotWhitespace => "\\S",
                Symbol::Alphabet => "[a-zA-Z]",
                Symbol::Char => ".",
                Symbol::Digit => "\\d",
                Symbol::NotDigit => "\\D",
                Symbol::Word => "\\w",
                Symbol::NotWord => "\\W",
                Symbol::Feed => "\\f",
                Symbol::Backspace => "[\\b]",
                Symbol::Boundary => "\\b",
            };
            output.push_str(transformed_symbol);
        }
        Node::Range(range) => match range {
            Range::AsciiRange(range) => {
                if range.negative {
                    output.push_str(format!("[^{}-{}]", range.start, range.end).as_str());
                } else {
                    output.push_str(format!("[{}-{}]", range.start, range.end).as_str());
                }
            }
            Range::NumericRange(range) => {
                if range.negative {
                    output.push_str(format!("[^{}-{}]", range.start, range.end).as_str());
                } else {
                    output.push_str(format!("[{}-{}]", range.start, range.end).as_str());
                }
            }
        },
        Node::Assertion(assertion) => {
            match assertion.kind {
                AssertionKind::Ahead => {
                    let body_source = to_source(&assertion.statements);
                    if assertion.negative {
                        output.push_str(format!("(?!{body_source})").as_str())
                    } else {
                        output.push_str(format!("(?={body_source})").as_str())
                    }
                }
                AssertionKind::Behind => {
                    let body_source = to_source(&assertion.statements);
                    if assertion.negative {
                        output.push_str(format!("(?<!{body_source})").as_str())
                    } else {
                        output.push_str(format!("(?<={body_source})").as_str())
                    }
                }
            };
        }
        Node::SpecialSymbol(special_symbol) => match special_symbol {
            SpecialSymbol::Start => output.push('^'),
            SpecialSymbol::End => output.push('$'),
        },
        Node::NegativeCharClass(class) => output.push_str(format!("[^{}]", class).as_str()),
        Node::EndOfInput => {}
    }
    output
}

fn expression_to_source(expression: &Expression) -> String {
    let mut output = String::new();
    match expression {
        Expression::Group(group) => match group.kind {
            GroupKind::Match => {
                let body_source = to_source(&group.statements);
                output.push_str(format!("(?:{body_source})").as_str())
            }
            GroupKind::Capture => {
                let body_source = to_source(&group.statements);
                if group.ident.is_some() {
                    output.push_str(
                        format!("(?<{}>{body_source})", group.ident.as_ref().unwrap()).as_str(),
                    )
                } else {
                    output.push_str(format!("({body_source})").as_str())
                }
            }
            GroupKind::Either => {
                let body_source = group
                    .statements
                    .iter()
                    .map(node_to_source)
                    .collect::<Vec<String>>()
                    .join("|");
                output.push_str(format!("(?:{body_source})").as_str())
            }
        },

        Expression::Atom(atom) => output.push_str(atom.as_str()),
        Expression::Range(range) => match range {
            Range::AsciiRange(range) => {
                output.push_str(format!("[{}-{}]", range.start, range.end).as_str());
            }
            Range::NumericRange(range) => {
                output.push_str(format!("[{}-{}]", range.start, range.end).as_str());
            }
        },
        Expression::Symbol(symbol) => {
            let transformed_symbol = match symbol {
                Symbol::Space => " ",
                Symbol::Newline => "\\n",
                Symbol::Vertical => "\\v",
                Symbol::Return => "\\r",
                Symbol::Tab => "\\t",
                Symbol::Null => "\\0",
                Symbol::Whitespace => "\\s",
                Symbol::NotWhitespace => "\\S",
                Symbol::Alphabet => "[a-zA-Z]",
                Symbol::Char => ".",
                Symbol::Digit => "\\d",
                Symbol::NotDigit => "\\D",
                Symbol::Word => "\\w",
                Symbol::NotWord => "\\W",
                Symbol::Feed => "\\f",
                Symbol::Backspace => "[\\b]",
                Symbol::Boundary => "\\b",
            };
            output.push_str(transformed_symbol);
        }
        Expression::NegativeCharClass(class) => output.push_str(format!("[^{}]", class).as_str()),
    }
    output
}
