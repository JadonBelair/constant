use std::collections::HashMap;

use crate::{
    error::ConstantError,
    lexer::{Token, TokenType},
};
pub use ast::{SingleOpType, DoubleOpType, Statement, Value};
use lazy_static::lazy_static;

mod ast;

lazy_static! {
    static ref DOUBLE_OPERATIONS: HashMap<TokenType, DoubleOpType> = {
        let mut h = HashMap::new();
        h.insert(TokenType::Plus, DoubleOpType::Add);
        h.insert(TokenType::Minus, DoubleOpType::Sub);
        h.insert(TokenType::Asterisk, DoubleOpType::Mul);
        h.insert(TokenType::Slash, DoubleOpType::Div);
        h.insert(TokenType::GT, DoubleOpType::GT);
        h.insert(TokenType::GTEq, DoubleOpType::GTEq);
        h.insert(TokenType::LT, DoubleOpType::LT);
        h.insert(TokenType::LTEq, DoubleOpType::LTEq);
        h.insert(TokenType::Eq, DoubleOpType::Eq);
        h.insert(TokenType::NotEq, DoubleOpType::NotEq);
        h.insert(TokenType::Swap, DoubleOpType::Swap);
        h
    };

    static ref SINGLE_OPERATIONS: HashMap<TokenType, SingleOpType> = {
        let mut h = HashMap::new();
        h.insert(TokenType::Print, SingleOpType::Print);
        h.insert(TokenType::Dup, SingleOpType::Dup);
        h.insert(TokenType::Drop, SingleOpType::Drop);
        h
    };
}

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    current_pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut tokens = tokens;
        if tokens.last() != Some(&Token::eof()) {
            tokens.push(Token::eof());
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
            let tok = self.current_token.clone();
            self.next();
            Ok(tok)
        } else {
            Err(ConstantError::NonMatchingToken(
                self.current_token.token_type,
                vec![token],
            ))
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_pos >= self.tokens.len() - 1
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ConstantError> {
        let mut ast = Vec::new();

        while self.current_token != Token::eof() {
            ast.push(self.statement()?);
        }
        ast.push(Statement::Empty);

        Ok(ast)
    }

    fn statement(&mut self) -> Result<Statement, ConstantError> {
        let res = if let Some(o) = SINGLE_OPERATIONS.get(&self.current_token.token_type) {
            Ok(Statement::SingleOperation(*o))
        } else if let Some(o) = DOUBLE_OPERATIONS.get(&self.current_token.token_type) {
            Ok(Statement::DoubleOperation(*o))
        } else if matches!(
            self.current_token.token_type,
            TokenType::Number | TokenType::Bool | TokenType::String
        ) {
            Ok(Statement::Push(Value::Literal(
                self.current_token.literal.clone().unwrap(),
            )))
        } else if self.check_token(TokenType::Bind) {
            self.match_token(TokenType::Bind)?;
            let ident = self.match_token(TokenType::Ident)?;
            Ok(Statement::Bind(ident.lexeme))
        } else if self.check_token(TokenType::Ident) {
            Ok(Statement::Push(Value::Ident(self.current_token.lexeme.clone())))
        } else if self.current_token.token_type == TokenType::EOF {
            Ok(Statement::Empty)
        } else {
            Err(ConstantError::NonMatchingToken(
                self.current_token.token_type,
                vec![
                    SINGLE_OPERATIONS.keys().map(|k| *k).collect::<Vec<TokenType>>(),
                    DOUBLE_OPERATIONS.keys().map(|k| *k).collect::<Vec<TokenType>>(),
                    vec![
                        TokenType::Number,
                        TokenType::Bool,
                        TokenType::String,
                    ],
                ]
                .concat(),
            ))
        };
        self.next();
        res
    }
}
