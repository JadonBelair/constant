use std::collections::HashMap;

use lazy_static::lazy_static;
pub use token::{Literal, Token, TokenType};

use crate::error::ConstantError;

mod token;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut h = HashMap::new();
        h.insert(String::from("true"), TokenType::Bool);
        h.insert(String::from("false"), TokenType::Bool);
        h.insert(String::from("print"), TokenType::Print);
        h.insert(String::from("dup"), TokenType::Dup);
        h.insert(String::from("swap"), TokenType::Swap);
        h.insert(String::from("drop"), TokenType::Drop);
        h.insert(String::from("bind"), TokenType::Bind);
        h.insert(String::from("if"), TokenType::If);
        h.insert(String::from("elif"), TokenType::Elif);
        h.insert(String::from("else"), TokenType::Else);
        h.insert(String::from("while"), TokenType::While);
        h.insert(String::from("proc"), TokenType::Proc);
        h.insert(String::from("call"), TokenType::Call);
        h.insert(String::from("do"), TokenType::Do);
        h.insert(String::from("end"), TokenType::End);
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
        let mut source = source.chars().collect::<Vec<char>>();
        if source.last() != Some(&'\0') {
            source.push('\0');
        }
        let current_char = source[0];

        Self {
            source,
            current_char,
            current_pos: 0,
        }
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
        if let Some(&c) = self.source.get(self.current_pos + 1) {
            self.current_pos += 1;
            self.current_char = c;
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
                Ok(Token::new(TokenType::Plus, '+'.into(), None))
            }
            '-' => {
                self.next();
                Ok(Token::new(TokenType::Minus, '-'.into(), None))
            }
            '*' => {
                self.next();
                Ok(Token::new(TokenType::Asterisk, '*'.into(), None))
            }
            '/' => {
                self.next();
                Ok(Token::new(TokenType::Slash, '/'.into(), None))
            }
            '%' => {
                self.next();
                Ok(Token::new(TokenType::Percent, '%'.into(), None))
            }
            '>' => {
                self.next();
                Ok(if self.current_char == '=' {
                    self.next();
                    Token::new(TokenType::GTEq, ">=".into(), None)
                } else if self.current_char.is_whitespace() || self.current_char == '\0' {
                    Token::new(TokenType::GT, ">".into(), None)
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
                    Token::new(TokenType::LTEq, "<=".into(), None)
                } else if self.current_char.is_whitespace() || self.current_char == '\0' {
                    Token::new(TokenType::LT, "<".into(), None)
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
                    Ok(Token::new(TokenType::Eq, "==".into(), None))
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
                    Ok(Token::new(TokenType::NotEq, "!=".into(), None))
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

                let text = self.source[start_pos..self.current_pos]
                    .iter()
                    .collect::<String>();

                let num = text.parse::<f32>().unwrap();

                Ok(Token::new(
                    TokenType::Number,
                    text,
                    Some(Literal::Number(num)),
                ))
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

                let text = self.source[start_pos..self.current_pos]
                    .iter()
                    .collect::<String>();
                let tok = Token::new(
                    TokenType::String,
                    self.source[(start_pos - 1)..=self.current_pos]
                        .iter()
                        .collect(),
                    Some(Literal::String(text)),
                );

                self.next(); // consumes the ending "

                Ok(tok)
            }
            'a'..='z' | 'A'..='Z' => {
                let start_pos = self.current_pos;

                while self.current_char.is_alphanumeric() || self.current_char == '_' {
                    self.next();
                }

                let text = self.source[start_pos..self.current_pos]
                    .iter()
                    .collect::<String>();

                let tt = *KEYWORDS.get(&text).unwrap_or(&TokenType::Ident);
                let literal = match text.as_str() {
                    "true" => Some(Literal::Bool(true)),
                    "false" => Some(Literal::Bool(false)),
                    _ => None,
                };

                Ok(Token::new(tt, text, literal))
            }
            '\0' => Ok(Token::eof()),
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
    fn lexer_next_token_number() -> Result<(), ConstantError> {
        let mut l = Lexer::new("123.456 123");

        assert!(l.next_token()?.literal.unwrap() == Literal::Number(123.456));
        assert!(l.next_token()?.literal.unwrap() == Literal::Number(123.0));
        assert!(l.next_token()?.token_type == TokenType::EOF);

        Ok(())
    }

    #[test]
    fn lexer_next_token_string() -> Result<(), ConstantError> {
        let mut l = Lexer::new("\"this is a test string\" \"this is another test string\"");

        assert!(
            l.next_token()?.literal.unwrap() == Literal::String("this is a test string".into())
        );
        assert!(
            l.next_token()?.literal.unwrap()
                == Literal::String("this is another test string".into())
        );
        assert!(l.next_token()?.token_type == TokenType::EOF);

        Ok(())
    }

    #[test]
    fn lexer_next_token_bool() -> Result<(), ConstantError> {
        let mut l = Lexer::new("true false true");

        assert!(l.next_token()?.literal.unwrap() == Literal::Bool(true));
        assert!(l.next_token()?.literal.unwrap() == Literal::Bool(false));
        assert!(l.next_token()?.literal.unwrap() == Literal::Bool(true));
        assert!(l.next_token()?.token_type == TokenType::EOF);

        Ok(())
    }

    #[test]
    fn lexer_next_token_built_in() -> Result<(), ConstantError> {
        let mut l = Lexer::new("print dup dup print");

        assert!(l.next_token()?.token_type == TokenType::Print);
        assert!(l.next_token()?.token_type == TokenType::Dup);
        assert!(l.next_token()?.token_type == TokenType::Dup);
        assert!(l.next_token()?.token_type == TokenType::Print);
        assert!(l.next_token()?.token_type == TokenType::EOF);

        Ok(())
    }

    #[test]
    fn lexer_skip_comments() -> Result<(), ConstantError> {
        let mut l = Lexer::new("// this is a comment");

        assert!(l.next_token()?.token_type == TokenType::EOF);

        Ok(())
    }
}
