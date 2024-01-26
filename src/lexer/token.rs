use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::error::ConstantError;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // operations
    Plus,
    Minus,
    Asterisk,
    Slash,
    GT,
    LT,
    Eq,
    GTEq,
    LTEq,
    NotEq,

    // data types
    Number(TokenValue),
    String(TokenValue),
    Bool(TokenValue),

    // built-ins
    Print,
    Dup,
    Swap,
    Drop,

    EOF,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum TokenValue {
    Number(f32),
    String(String),
    Bool(bool),
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(v) => f.write_fmt(format_args!("{v}")),
            Self::String(v) => f.write_fmt(format_args!("{v}")),
            Self::Bool(v) => f.write_fmt(format_args!("{v}")),
        }
    }
}

impl Add<TokenValue> for TokenValue {
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

impl Sub<TokenValue> for TokenValue {
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

impl Mul<TokenValue> for TokenValue {
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

impl Div<TokenValue> for TokenValue {
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
