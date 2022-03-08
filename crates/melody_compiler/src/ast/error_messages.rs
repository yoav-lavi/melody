use crate::errors::ParseError;

pub enum ErrorMessage {
    MissingRootNode,
    UnexpectedSpecialSymbolInQuantifier,
    UnexpectedQuantifierInQuantifier,
    UnexpectedAssertionInQuantifier,
    UnexpectedEmptyNodeInQuantifier,
    UnexpectedIdentifierForNonCaptureGroup,
    NegativeStartNotAllowed,
    NegativeEndNotAllowed,
    UninitializedVariable,
    UnexpectedVariableInvocationInQuantifier,
}

impl From<ErrorMessage> for ParseError {
    fn from(error: ErrorMessage) -> Self {
        let message = match error {
            ErrorMessage::MissingRootNode => "missing root node",
            ErrorMessage::UnexpectedSpecialSymbolInQuantifier => {
                "unexpected special symbol in quantifier"
            }
            ErrorMessage::UnexpectedQuantifierInQuantifier => "unexpected quantifier in quantifier",
            ErrorMessage::UnexpectedAssertionInQuantifier => "unexpected assertion in quantifier",
            ErrorMessage::UnexpectedEmptyNodeInQuantifier => "unexpected empty node in quantifier",
            ErrorMessage::UnexpectedVariableInvocationInQuantifier => {
                "unexpected variable invocation in quantifier"
            }
            ErrorMessage::UnexpectedIdentifierForNonCaptureGroup => {
                "unexpected identifier for non capture group"
            }
            ErrorMessage::NegativeStartNotAllowed => "negative start not allowed",
            ErrorMessage::NegativeEndNotAllowed => "negative end not allowed",
            ErrorMessage::UninitializedVariable => "usage of an uninitialized variable",
        };

        Self {
            message: String::from(message),
        }
    }
}
