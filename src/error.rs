use crate::lexer::TokenType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConstantError {
    #[error("Please provide source file")]
    NoSourceFile,

    #[error("Could not find provided source file '{0}'")]
    SourceFileNotFound(String),

    #[error(
        "Too many arguments, either pass the sourse file or run with no arguments for REPL mode"
    )]
    TooManyArgs,

    #[error("String is not terminated before end of file")]
    StringNotTerminated,

    #[error("Invalid string '{0}' at position {1}")]
    InvalidString(String, usize),

    #[error("{0} requires at least {1} items on the stack")]
    InvalidStackAmount(String, usize),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Unexpected Token: {0:?}")]
    UnexpectedToken(TokenType),

    #[error("Identifier '{0}' does not exist")]
    IdentDoesNotExist(String),

    #[error("Procedure '{0}' does not exist")]
    ProcDoesNotExist(String),
}
