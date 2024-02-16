use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
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
    And,
    Or,

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
    While,
    Proc,
    Call,
    Do,
    End,

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
                    Err(ConstantError::InvalidOperation(
                        "Can only add numbers to numbers".into(),
                    ))
                }
            }
            Self::Bool(_) => Err(ConstantError::InvalidOperation(
                "Cannot add booleans".into(),
            )),
            Self::String(s) => {
                if let Self::String(z) = rhs {
                    Ok(Self::String(s + &z))
                } else {
                    Err(ConstantError::InvalidOperation(
                        "Can only add strings to strings".into(),
                    ))
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
                    Err(ConstantError::InvalidOperation(
                        "Can only subtract numbers from numbers".into(),
                    ))
                }
            }
            Self::Bool(_) => Err(ConstantError::InvalidOperation(
                "Cannot subtract booleans".into(),
            )),
            Self::String(_) => Err(ConstantError::InvalidOperation(
                "Cannot subtract strings".into(),
            )),
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
                    Err(ConstantError::InvalidOperation(
                        "Can only multiply numbers with numbers".into(),
                    ))
                }
            }
            Self::Bool(_) => Err(ConstantError::InvalidOperation(
                "Cannot multiply booleans".into(),
            )),
            Self::String(s) => {
                if let Self::Number(n) = rhs {
                    Ok(Self::String(s.repeat(n as usize)))
                } else {
                    Err(ConstantError::InvalidOperation(
                        "Can only multiply strings with numbers".into(),
                    ))
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
                    Err(ConstantError::InvalidOperation(
                        "Can only divide numbers with number".into(),
                    ))
                }
            }
            Self::Bool(_) => Err(ConstantError::InvalidOperation(
                "Cannot divide with booleans".into(),
            )),
            Self::String(_) => Err(ConstantError::InvalidOperation(
                "Cannot divide with strings".into(),
            )),
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
                    Err(ConstantError::InvalidOperation(
                        "Can only mod numbers with numbers".into(),
                    ))
                }
            }
            Self::Bool(_) => Err(ConstantError::InvalidOperation(
                "Cannot mod with booleans".into(),
            )),
            Self::String(_) => Err(ConstantError::InvalidOperation(
                "Cannot mod with strings".into(),
            )),
        }
    }
}
