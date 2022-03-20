use super::utils::{mark_lazy, wrap_quantified};
use crate::ast::enums::{
    Assertion, AssertionKind, Expression, Group, GroupKind, MelodyAst, MelodyAstNode, Quantifier,
    QuantifierKind, Range, SpecialSymbol, Symbol, SymbolKind, VariableInvocation,
};

pub fn ast_to_regex(ast: &MelodyAst) -> String {
    match ast {
        MelodyAst::Root(nodes) => nodes.iter().map(node_to_regex).collect(),
        MelodyAst::Empty => String::new(),
    }
}

pub fn node_to_regex(node: &MelodyAstNode) -> String {
    match node {
        MelodyAstNode::Quantifier(quantifier) => transform_quantifier(quantifier),
        MelodyAstNode::Assertion(assertion) => transform_assertion(assertion),
        MelodyAstNode::SpecialSymbol(special_symbol) => transform_special_symbol(special_symbol),
        MelodyAstNode::Group(group) => transform_group(group),
        MelodyAstNode::Atom(atom) => String::from(atom),
        MelodyAstNode::Symbol(symbol) => transform_symbol(symbol),
        MelodyAstNode::Range(range) => transform_range(range),
        MelodyAstNode::NegativeCharClass(negative_char_class) => {
            transform_negative_char_class(negative_char_class)
        }
        MelodyAstNode::VariableInvocation(variable_invocation) => {
            transform_variable_invocation(variable_invocation)
        }
        MelodyAstNode::Skip => String::new(),
    }
}

fn expression_to_regex(expression: &Expression) -> String {
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
    let wrapped_expression = wrap_quantified(expression_to_regex(&quantifier.expression));
    let formatted_quantifier = match &quantifier.kind {
        QuantifierKind::Range { start, end } => format!("{}{{{start},{end}}}", wrapped_expression),
        QuantifierKind::Some => format!("{}+", wrapped_expression),
        QuantifierKind::Any => format!("{}*", wrapped_expression),
        QuantifierKind::Over(amount) => format!("{}{{{},}}", wrapped_expression, amount),
        QuantifierKind::Option => format!("{}?", wrapped_expression),
        QuantifierKind::Amount(amount) => format!("{}{{{amount}}}", wrapped_expression),
    };

    mark_lazy(formatted_quantifier, quantifier.lazy)
}

fn transform_assertion(assertion: &Assertion) -> String {
    let body_source = ast_to_regex(&assertion.statements);

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

fn transform_variable_invocation(variable_invocation: &VariableInvocation) -> String {
    ast_to_regex(&variable_invocation.statements)
}

fn transform_group(group: &Group) -> String {
    match group.kind {
        GroupKind::Match => {
            let body = ast_to_regex(&group.statements);
            format!("(?:{body})")
        }
        GroupKind::Capture => {
            let body = ast_to_regex(&group.statements);
            match group.ident.as_ref() {
                Some(ident) => format!("(?<{}>{body})", ident),
                None => format!("({body})"),
            }
        }
        GroupKind::Either => {
            let body = if let MelodyAst::Root(statements) = group.statements.as_ref() {
                statements
                    .iter()
                    .map(node_to_regex)
                    .collect::<Vec<String>>()
                    .join("|")
            } else {
                ast_to_regex(&group.statements)
            };
            format!("(?:{body})")
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
    let transformed_symbol = if symbol.negative {
        match symbol.kind {
            SymbolKind::Space => "[^ ]",
            SymbolKind::Newline => "[^\\n]",
            SymbolKind::Vertical => "[^\\v]",
            SymbolKind::Return => "[^\\r]",
            SymbolKind::Tab => "[^\\t]",
            SymbolKind::Null => "[^\\0]",
            SymbolKind::Whitespace => "\\S",
            SymbolKind::Alphabetic => "[^a-zA-Z]",
            SymbolKind::Alphanumeric => "[^a-zA-Z0-9]",
            SymbolKind::Char => "[^.]",
            SymbolKind::Digit => "\\D",
            SymbolKind::Word => "\\W",
            SymbolKind::Feed => "[^\\f]",
            SymbolKind::Backspace => "[^\\b]",
            SymbolKind::Boundary => "\\B",
        }
    } else {
        match symbol.kind {
            SymbolKind::Space => " ",
            SymbolKind::Newline => "\\n",
            SymbolKind::Vertical => "\\v",
            SymbolKind::Return => "\\r",
            SymbolKind::Tab => "\\t",
            SymbolKind::Null => "\\0",
            SymbolKind::Whitespace => "\\s",
            SymbolKind::Alphabetic => "[a-zA-Z]",
            SymbolKind::Alphanumeric => "[a-zA-Z0-9]",
            SymbolKind::Char => ".",
            SymbolKind::Digit => "\\d",
            SymbolKind::Word => "\\w",
            SymbolKind::Feed => "\\f",
            SymbolKind::Backspace => "[\\b]",
            SymbolKind::Boundary => "\\b",
        }
    };

    String::from(transformed_symbol)
}
