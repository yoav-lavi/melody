use super::consts::{NOT, SYMBOL_NAMESPACE_DELIMITER};
use super::types::ast::{MelodyAstNode, SpecialSymbolKind, Symbol, SymbolKind, UnicodeCategory, UnicodeCategoryKind};
use super::utils::first_last_inner_str;
use crate::ast::types::pest::Rule;
use crate::errors::CompilerError;
use crate::types::Result;
use pest::iterators::Pair;

pub fn symbol(pair: Pair<'_, Rule>) -> Result<MelodyAstNode> {
    let (negative, ident) = first_last_inner_str(pair)?;

    let negative = negative == NOT;

    if negative {
        match ident {
            "start" => return Err(CompilerError::NegativeStartNotAllowed),
            "end" => return Err(CompilerError::NegativeEndNotAllowed),
            "char" => return Err(CompilerError::NegativeCharNotAllowed),
            _ => {}
        }
    }

    if let Some((namespace, namespaced_ident)) = ident.split_once(SYMBOL_NAMESPACE_DELIMITER) {
        return match namespace {
            "category" => unicode_category(namespaced_ident, negative),
            _ => return Err(CompilerError::UnrecognizedSymbolNamespace),
        };
    }

    let symbol_node = match ident {
        // normal symbols
        "space" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Space,
            negative,
        }),
        "newline" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Newline,
            negative,
        }),
        "vertical" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Vertical,
            negative,
        }),
        "word" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Word,
            negative,
        }),
        "digit" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Digit,
            negative,
        }),
        "whitespace" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Whitespace,
            negative,
        }),
        "boundary" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Boundary,
            negative,
        }),
        "alphabetic" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Alphabetic,
            negative,
        }),
        "alphanumeric" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Alphanumeric,
            negative,
        }),
        "return" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Return,
            negative,
        }),
        "tab" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Tab,
            negative,
        }),
        "null" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Null,
            negative,
        }),
        "feed" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Feed,
            negative,
        }),
        "char" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Char,
            negative,
        }),
        "backspace" => MelodyAstNode::Symbol(Symbol {
            kind: SymbolKind::Backspace,
            negative,
        }),

        // special symbols
        "start" => MelodyAstNode::SpecialSymbol(SpecialSymbolKind::Start),
        "end" => MelodyAstNode::SpecialSymbol(SpecialSymbolKind::End),

        _ => return Err(CompilerError::UnrecognizedSymbol),
    };

    Ok(symbol_node)
}

#[allow(clippy::too_many_lines)]
fn unicode_category(ident: &str, negative: bool) -> Result<MelodyAstNode> {
    let unicode_group_node = match ident {
        // unicode categories
        "cased_letter" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::CasedLetter,
            negative,
        }),
        "close_punctuation" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::ClosePunctuation,
            negative,
        }),
        "connector_punctuation" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::ConnectorPunctuation,
            negative,
        }),
        "control" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::Control,
            negative,
        }),
        "currency_symbol" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::CurrencySymbol,
            negative,
        }),
        "dash_punctuation" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::DashPunctuation,
            negative,
        }),
        "decimal_digit_number" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::DecimalDigitNumber,
            negative,
        }),
        "enclosing_mark" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::EnclosingMark,
            negative,
        }),
        "final_punctuation" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::FinalPunctuation,
            negative,
        }),
        "format" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::Format,
            negative,
        }),
        "initial_punctuation" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::InitialPunctuation,
            negative,
        }),
        "letter_number" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::LetterNumber,
            negative,
        }),
        "letter" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::Letter,
            negative,
        }),
        "line_separator" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::LineSeparator,
            negative,
        }),
        "lowercase_letter" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::LowercaseLetter,
            negative,
        }),
        "mark" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::Mark,
            negative,
        }),
        "math_symbol" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::MathSymbol,
            negative,
        }),
        "modifier_letter" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::ModifierLetter,
            negative,
        }),
        "modifier_symbol" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::ModifierSymbol,
            negative,
        }),
        "non_spacing_mark" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::NonSpacingMark,
            negative,
        }),
        "number" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::Number,
            negative,
        }),
        "open_punctuation" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::OpenPunctuation,
            negative,
        }),
        "other_letter" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::OtherLetter,
            negative,
        }),
        "other_number" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::OtherNumber,
            negative,
        }),
        "other_punctuation" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::OtherPunctuation,
            negative,
        }),
        "other_symbol" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::OtherSymbol,
            negative,
        }),
        "other" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::Other,
            negative,
        }),
        "paragraph_separator" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::ParagraphSeparator,
            negative,
        }),
        "private_use" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::PrivateUse,
            negative,
        }),
        "punctuation" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::Punctuation,
            negative,
        }),
        "separator" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::Separator,
            negative,
        }),
        "space_separator" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::SpaceSeparator,
            negative,
        }),
        "spacing_combining_mark" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::SpacingCombiningMark,
            negative,
        }),
        "surrogate" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::Surrogate,
            negative,
        }),
        "symbol" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::Symbol,
            negative,
        }),
        "titlecase_letter" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::TitlecaseLetter,
            negative,
        }),
        "unassigned" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::Unassigned,
            negative,
        }),
        "uppercase_letter" => MelodyAstNode::UnicodeCategory(UnicodeCategory {
            kind: UnicodeCategoryKind::UppercaseLetter,
            negative,
        }),
        _ => return Err(CompilerError::UnrecognizedUnicodeCategory),
    };

    Ok(unicode_group_node)
}
