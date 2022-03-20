use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("could not parse an amount")]
    CouldNotParseAnAmount,
    #[error("usage of an invalid quantifier range")]
    InvalidQuantifierRange,
    #[error("encountered a missing positional node")]
    MissingNode,
    #[error("missing root node")]
    MissingRootNode,
    #[error("negative end not allowed")]
    NegativeEndNotAllowed,
    #[error("negative start not allowed")]
    NegativeStartNotAllowed,
    #[error("unexpected assertion in quantifier")]
    UnexpectedAssertionInQuantifier,
    #[error("unexpected identifier for non capture group")]
    UnexpectedIdentifierForNonCaptureGroup,
    #[error("unexpected quantifier in quantifier")]
    UnexpectedQuantifierInQuantifier,
    #[error("unexpected skipped node in quantifier")]
    UnexpectedSkippedNodeInQuantifier,
    #[error("unexpected special symbol in quantifier")]
    UnexpectedSpecialSymbolInQuantifier,
    #[error("unexpected variable invocation in quantifier")]
    UnexpectedVariableInvocationInQuantifier,
    #[error("usage of an uninitialized variable")]
    UninitializedVariable,
    #[error("usage of an unrecognized assertion")]
    UnrecognizedAssertion,
    #[error("usage of an unrecognized group")]
    UnrecognizedGroup,
    #[error("usage of unrecognized syntax")]
    UnrecognizedSyntax,
    #[error("usage of an unrecognized symbol")]
    UnrecognizedSymbol,
}
