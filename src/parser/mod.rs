use crate::{lexer::{Token, TokenType}, error::ConstantError};
pub use ast::Operation;

mod ast;

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    current_pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut tokens = tokens;
        match tokens.last() {
            Some(&Token {token_type: TokenType::EOF, ..}) => (),
            _ => tokens.push(Token::eof())
        }
        let current_token = tokens[0].clone();

        Self {
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

    fn check_token(&self, token: TokenType) -> bool {
        self.current_token.token_type == token
    }
    
    fn match_token(&mut self, token: TokenType) -> Result<Token, ConstantError> {
        if self.check_token(token.clone()) {
            self.next();
            Ok(self.current_token.clone())
        } else {
            Err(ConstantError::NonMatchingToken(self.current_token.token_type.clone(), token.into()))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Operation>, ConstantError> {
        let mut ast = Vec::new();

        Ok(ast)
    }
}
