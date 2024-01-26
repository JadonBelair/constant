use crate::lexer::TokenType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConstantError {
    #[error("Please provide source file")]
    NoSourceFile,

    #[error("Could not find provided source file '{0}'")]
    SourceFileNotFound(String),

    #[error("String is not terminated before end of file")]
    StringNotTerminated,

    #[error("Invalid string '{0}' at position {1}")]
    InvalidString(String, usize),

    #[error("{0} requires at least {1} items on the stack")]
    InvalidStackAmount(String, usize),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Expected one of {1:?}, found {0:?}")]
    NonMatchingToken(TokenType, Vec<TokenType>),
}
