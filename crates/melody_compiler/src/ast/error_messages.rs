use crate::errors::ParseError;

pub enum ErrorMessage {
    CouldNotParseAnAmount,
    InvalidQuantifierRange,
    MissingNode,
    MissingRootNode,
    NegativeEndNotAllowed,
    NegativeStartNotAllowed,
    UnexpectedAssertionInQuantifier,
    UnexpectedIdentifierForNonCaptureGroup,
    UnexpectedQuantifierInQuantifier,
    UnexpectedSkippedNodeInQuantifier,
    UnexpectedSpecialSymbolInQuantifier,
    UnexpectedVariableInvocationInQuantifier,
    UninitializedVariable,
    UnrecognizedAssertion,
    UnrecognizedGroup,
    UnrecognizedSyntax,
    UnrecognizedSymbol,
}

impl From<ErrorMessage> for ParseError {
    fn from(error: ErrorMessage) -> Self {
        let message = match error {
            ErrorMessage::CouldNotParseAnAmount => "could not parse an amount",
            ErrorMessage::InvalidQuantifierRange => "usage of an invalid quantifier range",
            ErrorMessage::MissingNode => "encountered a missing positional node",
            ErrorMessage::MissingRootNode => "missing root node",
            ErrorMessage::NegativeEndNotAllowed => "negative end not allowed",
            ErrorMessage::NegativeStartNotAllowed => "negative start not allowed",
            ErrorMessage::UnexpectedAssertionInQuantifier => "unexpected assertion in quantifier",
            ErrorMessage::UnexpectedIdentifierForNonCaptureGroup => {
                "unexpected identifier for non capture group"
            }
            ErrorMessage::UnexpectedQuantifierInQuantifier => "unexpected quantifier in quantifier",
            ErrorMessage::UnexpectedSkippedNodeInQuantifier => {
                "unexpected skipped node in quantifier"
            }
            ErrorMessage::UnexpectedSpecialSymbolInQuantifier => {
                "unexpected special symbol in quantifier"
            }
            ErrorMessage::UnexpectedVariableInvocationInQuantifier => {
                "unexpected variable invocation in quantifier"
            }
            ErrorMessage::UninitializedVariable => "usage of an uninitialized variable",
            ErrorMessage::UnrecognizedAssertion => "usage of an unrecognized assertion",
            ErrorMessage::UnrecognizedGroup => "usage of an unrecognized group",
            ErrorMessage::UnrecognizedSyntax => "usage of unrecognized syntax",
            ErrorMessage::UnrecognizedSymbol => "usage of an unrecognized symbol",
        };

        Self {
            message: String::from(message),
        }
    }
}
