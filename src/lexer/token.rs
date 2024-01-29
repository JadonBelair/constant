use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub, Rem},
};

use crate::error::ConstantError;

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
        }
    }

    pub fn eof() -> Self {
        Self {
            token_type: TokenType::EOF,
            lexeme: "".into(),
            literal: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum TokenType {
    // operations
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    GT,
    LT,
    Eq,
    GTEq,
    LTEq,
    NotEq,

    // data types
    Number,
    String,
    Bool,

    // built-ins
    Print,
    Dup,
    Swap,
    Drop,
    Bind,
    If,
    Elif,
    Else,
    EndIf,
    While,
    Do,
    EndWhile,

    Ident,

    EOF,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal {
    Number(f32),
    String(String),
    Bool(bool),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(v) => f.write_fmt(format_args!("{v}")),
            Self::String(v) => f.write_fmt(format_args!("{v}")),
            Self::Bool(v) => f.write_fmt(format_args!("{v}")),
        }
    }
}

impl Add<Literal> for Literal {
    type Output = Result<Self, ConstantError>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => {
                if let Self::Number(m) = rhs {
                    Ok(Self::Number(n + m))
                } else {
                    Err(ConstantError::InvalidOperation(String::from(
                        "Can only add numbers to numbers",
                    )))
                }
            }
            Self::Bool(_) => Err(ConstantError::InvalidOperation(String::from(
                "Cannot add booleans",
            ))),
            Self::String(s) => {
                if let Self::String(z) = rhs {
                    Ok(Self::String(s + &z))
                } else {
                    Err(ConstantError::InvalidOperation(String::from(
                        "Can only add strings to strings",
                    )))
                }
            }
        }
    }
}

impl Sub<Literal> for Literal {
    type Output = Result<Self, ConstantError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => {
                if let Self::Number(m) = rhs {
                    Ok(Self::Number(n - m))
                } else {
                    Err(ConstantError::InvalidOperation(String::from(
                        "Can only subtract numbers from numbers",
                    )))
                }
            }
            Self::Bool(_) => Err(ConstantError::InvalidOperation(String::from(
                "Cannot subtract booleans",
            ))),
            Self::String(_) => Err(ConstantError::InvalidOperation(String::from(
                "Cannot subtract strings",
            ))),
        }
    }
}

impl Mul<Literal> for Literal {
    type Output = Result<Self, ConstantError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => {
                if let Self::Number(m) = rhs {
                    Ok(Self::Number(n * m))
                } else {
                    Err(ConstantError::InvalidOperation(String::from(
                        "Can only multiply numbers with numbers",
                    )))
                }
            }
            Self::Bool(_) => Err(ConstantError::InvalidOperation(String::from(
                "Cannot multiply booleans",
            ))),
            Self::String(s) => {
                if let Self::Number(n) = rhs {
                    Ok(Self::String(s.repeat(n as usize)))
                } else {
                    Err(ConstantError::InvalidOperation(String::from(
                        "Can only multiply strings with numbers",
                    )))
                }
            }
        }
    }
}

impl Div<Literal> for Literal {
    type Output = Result<Self, ConstantError>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => {
                if let Self::Number(m) = rhs {
                    Ok(Self::Number(n / m))
                } else {
                    Err(ConstantError::InvalidOperation(String::from(
                        "Can only divide numbers with number",
                    )))
                }
            }
            Self::Bool(_) => Err(ConstantError::InvalidOperation(String::from(
                "Cannot divide with booleans",
            ))),
            Self::String(_) => Err(ConstantError::InvalidOperation(String::from(
                "Cannot divide with strings",
            ))),
        }
    }
}

impl Rem<Literal> for Literal {
    type Output = Result<Self, ConstantError>;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => {
                if let Self::Number(m) = rhs {
                    Ok(Self::Number(n % m))
                } else {
                    Err(ConstantError::InvalidOperation(String::from(
                        "Can only mod numbers with numbers"
                    )))
                }
            }
            Self::Bool(_) => Err(ConstantError::InvalidOperation(String::from(
                "Cannot mod with booleans",
            ))),
            Self::String(_) => Err(ConstantError::InvalidOperation(String::from(
                "Cannot mod with strings",
            ))),
        }
    }
}
