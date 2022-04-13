use crate::errors::CompilerError;

pub type Result<T> = core::result::Result<T, CompilerError>;
