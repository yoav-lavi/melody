use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("invalid arguments, please supply a path argument or use --repl")]
    MissingPath,
    #[error("unable read file at path {0}")]
    ReadFileError(String),
    #[error("{0}")]
    ParseError(String),
    #[error("{0}")]
    WriteFileError(String),
    #[error("unable to read input")]
    ReadInputError,
}
