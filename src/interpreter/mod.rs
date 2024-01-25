use crate::{
    error::ConstantError,
    lexer::{Token, TokenValue},
};

pub struct Interpreter<'a> {
    stack: Vec<TokenValue>,
    tokens: &'a Vec<Token>,
    current_token: Token,
    current_pos: usize,
}

impl<'a> Interpreter<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        let mut i = Self {
            stack: Vec::new(),
            tokens,
            current_token: Token::EOF,
            current_pos: 0,
        };
        if let Some(tok) = i.tokens.get(0) {
            i.current_token = tok.clone();
        }

        i
    }

    fn next(&mut self) {
        self.current_pos += 1.clamp(0, self.tokens.len());

        if self.current_pos >= self.tokens.len() {
            self.current_token = Token::EOF;
            self.current_pos = self.tokens.len();
        } else {
            self.current_token = self.tokens[self.current_pos].clone();
        }
    }

    #[allow(unused)] // get rid of compiler warnings until we need to use this
    fn peek(&self) -> Token {
        self.tokens
            .get(self.current_pos + 1)
            .unwrap_or(&Token::EOF)
            .clone()
    }

    pub fn interpret(&mut self) -> Result<(), ConstantError> {
        while self.current_token != Token::EOF {
            match &self.current_token {
                Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                    let action = match &self.current_token {
                        Token::Plus => "Addition",
                        Token::Minus => "Subtraction",
                        Token::Asterisk => "Multiplication",
                        Token::Slash => "Division",
                        _ => unreachable!(),
                    };

                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from(action), 2));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from(action), 2));
                    };

                    // find a way to combine with above that doesnt result in weird error about closures
                    let result = match &self.current_token {
                        Token::Plus => first + second,
                        Token::Minus => first - second,
                        Token::Asterisk => first * second,
                        Token::Slash => first / second,
                        _ => unreachable!(),
                    };

                    self.stack.push(result?);
                }
                Token::GT | Token::LT | Token::Eq | Token::GTEq | Token::LTEq | Token::NotEq => {
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(
                            String::from("Comparison"),
                            2,
                        ));
                    };
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(
                            String::from("Comparison"),
                            2,
                        ));
                    };

                    let operation = match &self.current_token {
                        Token::GT => |x: TokenValue, y: TokenValue| x > y,
                        Token::LT => |x: TokenValue, y: TokenValue| x < y,
                        Token::Eq => |x: TokenValue, y: TokenValue| x == y,
                        Token::GTEq => |x: TokenValue, y: TokenValue| x >= y,
                        Token::LTEq => |x: TokenValue, y: TokenValue| x <= y,
                        Token::NotEq => |x: TokenValue, y: TokenValue| x != y,
                        _ => unreachable!(),
                    };

                    self.stack.push(TokenValue::Bool(operation(first, second)));
                }
                Token::Number(v) | Token::String(v) | Token::Bool(v) => self.stack.push(v.clone()),
                Token::Ident(_) => {}
                Token::Print => {
                    println!(
                        "{}",
                        if let Some(val) = self.stack.pop() {
                            val
                        } else {
                            return Err(ConstantError::InvalidStackAmount(
                                String::from("Printing"),
                                1,
                            ));
                        }
                    )
                }
                Token::Dup => {
                    let value = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Duping"), 1));
                    };

                    self.stack.push(value.clone());
                    self.stack.push(value);
                }
                Token::Swap => {
                    let first = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(
                            String::from("Swapping"),
                            2,
                        ));
                    };
                    let second = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(
                            String::from("Swapping"),
                            2,
                        ));
                    };

                    self.stack.push(first);
                    self.stack.push(second);
                }
                Token::Drop => {
                    if self.stack.pop().is_none() {
                        return Err(ConstantError::InvalidStackAmount(
                            String::from("Dropping"),
                            1,
                        ));
                    }
                }
                Token::EOF => (),
            }

            self.next();
        }

        Ok(())
    }
}
