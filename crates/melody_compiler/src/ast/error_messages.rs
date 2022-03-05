use crate::errors::ParseError;

pub enum ErrorMessage {
    MissingRootNode,
    UnexpectedSpecialSymbolInQuantifier,
    UnexpectedQuantifierInQuantifier,
    UnexpectedAssertionInQuantifier,
    UnexpectedEndOfInputInQuantifier,
    UnexpectedIdetifierForNonCaptureGroup,
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
            ErrorMessage::UnexpectedIdetifierForNonCaptureGroup => {
                "unexpected identifier for non capture group"
            }
        };

        ParseError {
            message: String::from(message),
        }
    }
}
