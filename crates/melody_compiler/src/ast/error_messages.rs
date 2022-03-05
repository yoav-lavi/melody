use crate::errors::ParseError;

pub enum ErrorMessage {
    MissingRootNode,
    UnexpectedSpecialSymbolInQuantifier,
    UnexpectedQuantifierInQuantifier,
    UnexpectedAssertionInQuantifier,
    UnexpectedEndOfInputInQuantifier,
    UnexpectedIdentifierForNonCaptureGroup,
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
            ErrorMessage::UnexpectedEndOfInputInQuantifier => {
                "unexpected end of input in quantifier"
            }
            ErrorMessage::UnexpectedIdentifierForNonCaptureGroup => {
                "unexpected identifier for non capture group"
            }
        };

        Self {
            message: String::from(message),
        }
    }
}
