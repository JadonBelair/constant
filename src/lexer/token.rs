use std::{ops::{Add, Sub, Mul, Div}, fmt::Display};

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
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => {
                if let Self::Number(m) = rhs {
                    Self::Number(n + m)
                } else {
                    panic!("Error: Can only add numbers to numbers");
                }
            },
            Self::Bool(_) => panic!("Error: Cannot add bool"),
            Self::String(s) => {
                if let Self::String(z) = rhs {
                    Self::String(s+&z)
                } else {
                    panic!("Error: Can only add strings to strings");
                }
            }
        }
    }
}

impl Sub<TokenValue> for TokenValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => {
                if let Self::Number(m) = rhs {
                    Self::Number(n - m)
                } else {
                    panic!("Error: Can only subtract numbers to numbers");
                }
            },
            Self::Bool(_) => panic!("Error: Cannot subtract bool"),
            Self::String(_) => {
                panic!("Error: Cannot subtract strings");
            }
        }
    }
}

impl Mul<TokenValue> for TokenValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => {
                if let Self::Number(m) = rhs {
                    Self::Number(n*m)
                } else {
                    panic!("Error: Can only multiply numbers to numbers");
                }
            },
            Self::Bool(_) => panic!("Error: Cannot subtract bool"),
            Self::String(s) => {
                if let Self::Number(n) = rhs {
                    Self::String(s.repeat(n as usize))
                } else {
                    panic!("Error: Can only multiply strings with numbers");
                }
            }
        }
    }
}

impl Div<TokenValue> for TokenValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::Number(n) => {
                if let Self::Number(m) = rhs {
                    Self::Number(n/m)
                } else {
                    panic!("Error: Can only divide numbers with numbers");
                }
            },
            Self::Bool(_) => panic!("Error: Cannot divide with booleans"),
            Self::String(_) => panic!("Error: Cannot divide with strings"),
        }
    }
}
