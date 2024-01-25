use crate::{lexer::{TokenValue, Token}, error::ConstantError};

pub struct Interpreter<'a> {
    stack: Vec<TokenValue>,
    tokens: &'a Vec<Token>,
}

impl<'a> Interpreter<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            stack: Vec::new(),
            tokens
        }
    }

    pub fn interpret(&mut self) -> Result<(), ConstantError> {
        for token in self.tokens {
            match token {
                Token::Plus => {
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Addition"), 2));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Addition"), 2));
                    };

                    self.stack.push(first + second);
                },
                Token::Minus => {
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Subtraction"), 2));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Subtraction"), 2));
                    };

                    self.stack.push(first - second);
                },
                Token::Asterisk => {
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Multiplication"), 2));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Multiplication"), 2));
                    };

                    self.stack.push(first * second);
                },
                Token::Slash => {
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Division"), 2));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Division"), 2));
                    };

                    self.stack.push(first / second);
                },
                Token::GT => {
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };

                    self.stack.push(TokenValue::Bool(first > second));
                },
                Token::LT => {
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };

                    self.stack.push(TokenValue::Bool(first < second));
                },
                Token::Eq => {
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };

                    self.stack.push(TokenValue::Bool(first == second));
                },
                Token::GTEq => {
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };

                    self.stack.push(TokenValue::Bool(first >= second));
                },
                Token::LTEq => {
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };

                    self.stack.push(TokenValue::Bool(first <= second));
                },
                Token::NotEq => {
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Comparison"), 2));
                    };

                    self.stack.push(TokenValue::Bool(first != second));
                },
                Token::Number(v) | Token::String(v) | Token::Bool(v) => self.stack.push(v.clone()),
                Token::Print => {
                    println!("{}", if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Printing"), 1));
                    })
                },
                Token::Dup => {
                    let value = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Duping"), 1));
                    };

                    self.stack.push(value.clone());
                    self.stack.push(value);
                },
                Token::Swap => {
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Swapping"), 2));
                    };
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Swapping"), 2));
                    };
                    
                    self.stack.push(first);
                    self.stack.push(second);
                },
                Token::Drop => {
                    if self.stack.pop().is_none() {
                        return Err(ConstantError::InvalidStackAmount(String::from("Dropping"), 1));
                    }
                }
                Token::EOF => ()
            }
        }

        Ok(())
    }
}
