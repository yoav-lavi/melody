use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    /// returned when `over` receives an amount that does not parse correctly
    #[error("could not parse an amount")]
    CouldNotParseAnAmount,
    /// returned when a quantifier range (`3 to 5 of ...`) does not
    /// parse correctly or when the start of the range is larger then the end
    #[error("usage of an invalid quantifier range")]
    InvalidQuantifierRange,
    /// returned when an expected positional node does not exist
    #[error("encountered a missing positional node")]
    MissingNode,
    /// returned if the root node does not exist
    #[error("missing root node")]
    MissingRootNode,
    /// returned if `not <end>` is encountered
    #[error("negative end not allowed")]
    NegativeEndNotAllowed,
    /// returned if `not <start>` is encountered
    #[error("negative start not allowed")]
    NegativeStartNotAllowed,
    /// returned if the given input cannot be parsed
    #[error("{0}")]
    ParseError(String),
    /// returned if an assertion is quantified
    #[error("unexpected assertion in quantifier")]
    UnexpectedAssertionInQuantifier,
    /// returned if a non capture group (e.g. match) has an identifier
    #[error("unexpected identifier for non capture group")]
    UnexpectedIdentifierForNonCaptureGroup,
    /// returned if a quantfier is nested
    #[error("unexpected quantifier in quantifier")]
    UnexpectedQuantifierInQuantifier,
    /// (unreachable) returned if a skipped node (currently only EOF) is quantified
    #[error("unexpected skipped node in quantifier")]
    UnexpectedSkippedNodeInQuantifier,
    /// returned if a special symbol (`<start>` or `<end>`) is quantified
    #[error("unexpected special symbol in quantifier")]
    UnexpectedSpecialSymbolInQuantifier,
    /// returned if a variable invocation is quantified
    #[error("unexpected variable invocation in quantifier")]
    UnexpectedVariableInvocationInQuantifier,
    /// returned if a variable invocation is not preceeded by a declaration
    #[error("usage of an uninitialized variable")]
    UninitializedVariable,
    /// (unreachable) returned if an assertion is not of a recognized kind
    #[error("usage of an unrecognized assertion")]
    UnrecognizedAssertion,
    /// (unreachable) returned if a group is not of a recognized kind
    #[error("usage of an unrecognized group")]
    UnrecognizedGroup,
    /// (unreachable) returned if any parsed syntax is not recognized
    #[error("usage of unrecognized syntax")]
    UnrecognizedSyntax,
    /// returned if any parsed symbol is not recognized
    #[error("usage of an unrecognized symbol")]
    UnrecognizedSymbol,
    /// returned if any parsed symbol namespace is not recognized
    #[error("usage of an unrecognized symbol namespace")]
    UnrecognizedSymbolNamespace,
    /// returned if any parsed unicode category
    #[error("usage of an unrecognized unicode category")]
    UnrecognizedUnicodeCategory,
}
