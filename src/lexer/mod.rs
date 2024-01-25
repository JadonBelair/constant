use std::collections::HashMap;

use lazy_static::lazy_static;
pub use token::{Token, TokenValue};

use crate::error::ConstantError;

pub mod token;

lazy_static! {
    static ref KEYWORDS: HashMap<String, Token> = {
        let mut h = HashMap::new();
        h.insert(String::from("true"), Token::Bool(TokenValue::Bool(true)));
        h.insert(String::from("false"), Token::Bool(TokenValue::Bool(false)));
        h.insert(String::from("print"), Token::Print);
        h.insert(String::from("dup"), Token::Dup);
        h.insert(String::from("swap"), Token::Swap);
        h.insert(String::from("drop"), Token::Drop);
        h
    };
}

pub struct Lexer {
    source: Vec<char>,
    current_char: char,
    current_pos: usize,
}

impl Lexer {
    pub fn new<'a>(source: &'a str) -> Self {
        let mut l = Self {
            source: source.chars().collect(),
            current_char: '\0',
            current_pos: 0,
        };
        if let Some(&c) = l.source.get(0) {
            l.current_char = c;
        }

        l
    }

    fn skip_whitespace(&mut self) -> bool {
        let mut skipped = false;
        while self.current_char.is_whitespace() {
            self.next();
            skipped = true;
        }

        skipped
    }

    fn skip_comments(&mut self) -> bool {
        let mut skipped = false;
        if self.current_char == '/' && self.peek() == '/' {
            skipped = true;
            self.next();
            while self.current_char != '\n' && self.current_char != '\0' {
                self.next();
            }
        }

        skipped
    }

    fn next(&mut self) {
        self.current_pos += 1.clamp(0, self.source.len());

        if self.current_pos >= self.source.len() {
            self.current_char = '\0';
            self.current_pos = self.source.len();
        } else {
            self.current_char = self.source[self.current_pos];
        }
    }

    fn peek(&self) -> char {
        *self.source.get(self.current_pos + 1).unwrap_or(&'\0')
    }

    pub fn next_token(&mut self) -> Result<Token, ConstantError> {
        while self.skip_comments() || self.skip_whitespace() {}

        match self.current_char {
            '+' => {
                self.next();
                Ok(Token::Plus)
            }
            '-' => {
                self.next();
                Ok(Token::Minus)
            }
            '*' => {
                self.next();
                Ok(Token::Asterisk)
            }
            '/' => {
                self.next();
                Ok(Token::Slash)
            }
            '>' => {
                self.next();
                Ok(if self.current_char == '=' {
                    self.next();
                    Token::GTEq
                } else if self.current_char.is_whitespace() || self.current_char == '\0' {
                    Token::GT
                } else {
                    return Err(ConstantError::InvalidString(
                        format!(">{}", self.current_char),
                        self.current_pos,
                    ));
                })
            }
            '<' => {
                self.next();
                Ok(if self.current_char == '=' {
                    self.next();
                    Token::LTEq
                } else if self.current_char.is_whitespace() || self.current_char == '\0' {
                    Token::LT
                } else {
                    return Err(ConstantError::InvalidString(
                        format!("<{}", self.current_char),
                        self.current_pos - 1,
                    ));
                })
            }
            '=' => {
                self.next();
                if self.current_char == '=' {
                    self.next();
                    Ok(Token::Eq)
                } else {
                    Err(ConstantError::InvalidString(
                        format!("={}", self.current_char),
                        self.current_pos - 1,
                    ))
                }
            }
            '!' => {
                self.next();
                if self.current_char == '=' {
                    self.next();
                    Ok(Token::NotEq)
                } else {
                    Err(ConstantError::InvalidString(
                        format!("!{}", self.current_char),
                        self.current_pos - 1,
                    ))
                }
            }
            '0'..='9' => {
                let start_pos = self.current_pos;
                while self.current_char.is_numeric() {
                    self.next();
                }

                if self.current_char == '.' {
                    self.next();

                    while self.current_char.is_numeric() {
                        self.next();
                    }
                }

                let num = self.source[start_pos..self.current_pos]
                    .iter()
                    .collect::<String>()
                    .parse::<f32>()
                    .unwrap();
                Ok(Token::Number(TokenValue::Number(num)))
            }
            '"' => {
                self.next();
                let start_pos = self.current_pos;

                while self.current_char != '"' && self.current_char != '\0' {
                    self.next();
                }

                if self.current_char == '\0' {
                    return Err(ConstantError::StringNotTerminated);
                }

                let tok = Token::String(TokenValue::String(
                    self.source[start_pos..self.current_pos].iter().collect(),
                ));
                self.next(); // consumes the ending "

                Ok(tok)
            }
            'a'..='z' | 'A'..='Z' => {
                let start_pos = self.current_pos;

                while !self.current_char.is_whitespace() && self.current_char != '\0' {
                    self.next();
                }

                let text = self.source[start_pos..self.current_pos].iter().collect();
                if let Some(k) = KEYWORDS.get(&text) {
                    Ok(k.clone())
                } else {
                    Err(ConstantError::InvalidString(text, start_pos))
                }
            }
            '\0' => Ok(Token::EOF),
            _ => Err(ConstantError::InvalidString(
                format!("{}", self.current_char),
                self.current_pos,
            )),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ConstantError> {
        let mut tokens = Vec::new();

        while self.current_char != '\0' {
            tokens.push(self.next_token()?);
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn lexer_next() {
        let mut l = Lexer::new("ah");

        assert!(l.current_char == 'a');
        l.next();
        assert!(l.current_char == 'h');
        l.next();
        assert!(l.current_char == '\0');
        l.next();
        assert!(l.current_char == '\0');
    }

    #[test]
    fn lexer_next_token_number() -> Result<()> {
        let mut l = Lexer::new("123.456 123");

        assert!(l.next_token()? == Token::Number(TokenValue::Number(123.456)));
        assert!(l.next_token()? == Token::Number(TokenValue::Number(123.0)));
        assert!(l.next_token()? == Token::EOF);

        Ok(())
    }

    #[test]
    fn lexer_next_token_string() -> Result<()> {
        let mut l = Lexer::new("\"this is a test string\" \"this is another test string\"");

        assert!(
            l.next_token()?
                == Token::String(TokenValue::String(String::from("this is a test string")))
        );
        assert!(
            l.next_token()?
                == Token::String(TokenValue::String(String::from(
                    "this is another test string"
                )))
        );
        assert!(l.next_token()? == Token::EOF);

        Ok(())
    }

    #[test]
    fn lexer_next_token_bool() -> Result<()> {
        let mut l = Lexer::new("true false true");

        assert!(l.next_token()? == Token::Bool(TokenValue::Bool(true)));
        assert!(l.next_token()? == Token::Bool(TokenValue::Bool(false)));
        assert!(l.next_token()? == Token::Bool(TokenValue::Bool(true)));
        assert!(l.next_token()? == Token::EOF);

        Ok(())
    }

    #[test]
    fn lexer_next_token_built_in() -> Result<()> {
        let mut l = Lexer::new("print dup dup print");

        assert!(l.next_token()? == Token::Print);
        assert!(l.next_token()? == Token::Dup);
        assert!(l.next_token()? == Token::Dup);
        assert!(l.next_token()? == Token::Print);
        assert!(l.next_token()? == Token::EOF);

        Ok(())
    }

    #[test]
    fn lexer_skip_comments() -> Result<()> {
        let mut l = Lexer::new("// this is a comment");

        assert!(l.next_token()? == Token::EOF);

        Ok(())
    }
}
