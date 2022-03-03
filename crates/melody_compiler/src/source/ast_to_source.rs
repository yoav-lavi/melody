use super::utils::wrap_quantified;
use crate::ast::enums::{
    Assertion, AssertionKind, Expression, Group, GroupKind, Node, Quantifier, QuantifierKind,
    Range, SpecialSymbol, Symbol,
};

pub fn to_source(ast: &[Node]) -> String {
    let mut output = String::new();
    for node in ast {
        output.push_str(node_to_source(node).as_str());
    }
    output
}

fn node_to_source(node: &Node) -> String {
    match node {
        Node::Quantifier(quantifier) => transform_quantifier(quantifier),
        Node::Assertion(assertion) => transform_assertion(assertion),
        Node::SpecialSymbol(special_symbol) => transform_special_symbol(special_symbol),
        Node::Group(group) => transform_group(group),
        Node::Atom(atom) => String::from(atom),
        Node::Symbol(symbol) => transform_symbol(symbol),
        Node::Range(range) => transform_range(range),
        Node::NegativeCharClass(negative_char_class) => {
            transform_negative_char_class(negative_char_class)
        }
        Node::EndOfInput => String::new(),
    }
}

fn expression_to_source(expression: &Expression) -> String {
    match expression {
        Expression::Group(group) => transform_group(group),
        Expression::Atom(atom) => String::from(atom),
        Expression::Range(range) => transform_range(range),
        Expression::Symbol(symbol) => transform_symbol(symbol),
        Expression::NegativeCharClass(negative_char_class) => {
            transform_negative_char_class(negative_char_class)
        }
    }
}

fn transform_special_symbol(special_symbol: &SpecialSymbol) -> String {
    let transformed_special_symbol = match special_symbol {
        SpecialSymbol::Start => '^',
        SpecialSymbol::End => '$',
    };

    String::from(transformed_special_symbol)
}

fn transform_quantifier(quantifier: &Quantifier) -> String {
    let wrapped_expression = wrap_quantified(expression_to_source(&quantifier.expression));
    match &quantifier.kind {
        QuantifierKind::Range { start, end } => format!("{}{{{start},{end}}}", wrapped_expression),
        QuantifierKind::Some => format!("{}+", wrapped_expression),
        QuantifierKind::Any => format!("{}*", wrapped_expression),
        QuantifierKind::Over(amount) => format!(
            "{}{{{},}}",
            wrapped_expression,
            amount.parse::<usize>().unwrap() + 1
        ),
        QuantifierKind::Option => format!("{}?", wrapped_expression),
        QuantifierKind::Amount(amount) => format!("{}{{{amount}}}", wrapped_expression),
    }
}

fn transform_assertion(assertion: &Assertion) -> String {
    let body_source = to_source(&assertion.statements);

    match assertion.kind {
        AssertionKind::Ahead => {
            if assertion.negative {
                format!("(?!{body_source})")
            } else {
                format!("(?={body_source})")
            }
        }
        AssertionKind::Behind => {
            if assertion.negative {
                format!("(?<!{body_source})")
            } else {
                format!("(?<={body_source})")
            }
        }
    }
}

fn transform_negative_char_class(class: &str) -> String {
    format!("[^{}]", class)
}

fn transform_group(group: &Group) -> String {
    match group.kind {
        GroupKind::Match => {
            let body_source = to_source(&group.statements);
            format!("(?:{body_source})")
        }
        GroupKind::Capture => {
            let body_source = to_source(&group.statements);
            if group.ident.is_some() {
                format!("(?<{}>{body_source})", group.ident.as_ref().unwrap())
            } else {
                format!("({body_source})")
            }
        }
        GroupKind::Either => {
            let body_source = group
                .statements
                .iter()
                .map(node_to_source)
                .collect::<Vec<String>>()
                .join("|");
            format!("(?:{body_source})")
        }
    }
}

fn transform_range(range: &Range) -> String {
    match range {
        Range::AsciiRange(range) => {
            if range.negative {
                format!("[^{}-{}]", range.start, range.end)
            } else {
                format!("[{}-{}]", range.start, range.end)
            }
        }
        Range::NumericRange(range) => {
            if range.negative {
                format!("[^{}-{}]", range.start, range.end)
            } else {
                format!("[{}-{}]", range.start, range.end)
            }
        }
    }
}

fn transform_symbol(symbol: &Symbol) -> String {
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

    String::from(transformed_symbol)
}
