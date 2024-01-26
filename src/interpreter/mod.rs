use std::io::Write;

use crate::{
    error::ConstantError,
    lexer::{Lexer, Token, TokenType, Literal},
};

pub struct Interpreter {
    stack: Vec<Literal>,
    tokens: Vec<Token>,
    current_token: Token,
    current_pos: usize,
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut tokens = tokens;
        if tokens.last() != Some(&Token::eof()) {
            tokens.push(Token::eof());
        }
        let current_token = tokens[0].clone();

        Self {
            stack: Vec::new(),
            tokens,
            current_token,
            current_pos: 0,
        }
    }

    fn next(&mut self) {
        if let Some(t) = self.tokens.get(self.current_pos + 1) {
            self.current_pos += 1;
            self.current_token = t.clone();
        }
    }

    #[allow(unused)] // get rid of compiler warnings until we need to use this
    fn peek(&self) -> Token {
        self.tokens
            .get(self.current_pos + 1)
            .unwrap_or(&Token::eof())
            .clone()
    }

    pub fn interpret(&mut self) -> Result<(), ConstantError> {
        while self.current_token.token_type != TokenType::EOF {
            match &self.current_token.token_type {
                TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash => {
                    let action = match &self.current_token.token_type {
                        TokenType::Plus => "Addition",
                        TokenType::Minus => "Subtraction",
                        TokenType::Asterisk => "Multiplication",
                        TokenType::Slash => "Division",
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
                        // we restore the stack on a failed operation
                        // doens't really matter for interpreted mode 
                        // but it's a nice feature to have in the REPL
                        self.stack.push(second);
                        return Err(ConstantError::InvalidStackAmount(String::from(action), 2));
                    };

                    // find a way to combine with above that doesnt result in weird error about closures
                    let operation = |first: Literal, second: Literal| {
                        match &self.current_token.token_type {
                            TokenType::Plus => first + second,
                            TokenType::Minus => first - second,
                            TokenType::Asterisk => first * second,
                            TokenType::Slash => first / second,
                            _ => unreachable!(),
                        }
                    };

                    match operation(first.clone(), second.clone()) {
                        Ok(v) => self.stack.push(v),
                        Err(e) => {
                            self.stack.push(first);
                            self.stack.push(second);
                            return Err(e);
                        }
                    }
                }
                TokenType::GT | TokenType::LT | TokenType::Eq | TokenType::GTEq | TokenType::LTEq | TokenType::NotEq => {
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
                        self.stack.push(second);
                        return Err(ConstantError::InvalidStackAmount(
                            String::from("Comparison"),
                            2,
                        ));
                    };

                    let operation = |x: Literal, y: Literal | {
                        match &self.current_token.token_type {
                            TokenType::GT => x > y,
                            TokenType::LT => x < y,
                            TokenType::Eq => x == y,
                            TokenType::GTEq => x >= y,
                            TokenType::LTEq => x <= y,
                            TokenType::NotEq => x != y,
                            _ => unreachable!(),
                        }
                    };
                    self.stack.push(Literal::Bool(operation(first, second)));
                }
                TokenType::Number | TokenType::String | TokenType::Bool => self.stack.push(self.current_token.literal.clone().unwrap()),
                TokenType::Print => {
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
                TokenType::Dup => {
                    let value = if let Some(val) = self.stack.pop() {
                        val
                    } else {
                        return Err(ConstantError::InvalidStackAmount(String::from("Duping"), 1));
                    };

                    self.stack.push(value.clone());
                    self.stack.push(value);
                }
                TokenType::Swap => {
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
                        self.stack.push(first);
                        return Err(ConstantError::InvalidStackAmount(
                            String::from("Swapping"),
                            2,
                        ));
                    };

                    self.stack.push(first);
                    self.stack.push(second);
                }
                TokenType::Drop => {
                    if self.stack.pop().is_none() {
                        return Err(ConstantError::InvalidStackAmount(
                            String::from("Dropping"),
                            1,
                        ));
                    }
                }
                TokenType::EOF => (),
            }

            self.next();
        }

        Ok(())
    }

    pub fn repl(&mut self) {
        println!("Welcome to the Constant REPL, type 'exit' or 'quit' to quit");
        loop {
            print!("> ");
            std::io::stdout()
                .flush()
                .expect("Error: Could not flush stdout");

            let mut code = String::new();
            std::io::stdin()
                .read_line(&mut code)
                .expect("Error: Could not read input");

            if code.trim() == "exit" || code.trim() == "quit" {
                return;
            }

            match Lexer::new(&code).tokenize() {
                Ok(tokens) => self.tokens = tokens,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };
            self.current_pos = 0;
            self.current_token = self.tokens[0].clone();

            if let Err(e) = self.interpret() {
                println!("{e}");
            }
        }
    }
}
