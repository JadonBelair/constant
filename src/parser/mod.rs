use std::collections::HashMap;

use crate::{
    error::ConstantError,
    lexer::{Token, TokenType},
};
pub use ast::{DoubleOpType, SingleOpType, Statement, Value};
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
    previous_token: Token,
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
            current_token: current_token.clone(),
            previous_token: current_token,
            current_pos: 0,
        }
    }

    fn next(&mut self) {
        if let Some(t) = self.tokens.get(self.current_pos + 1) {
            self.current_pos += 1;
            self.previous_token = self.current_token.clone();
            self.current_token = t.clone();
        }
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

    pub fn parse(&mut self) -> Result<Vec<Statement>, ConstantError> {
        let mut ast = Vec::new();

        while !self.check_token(TokenType::EOF) {
            ast.push(self.statement()?);
        }
        ast.push(Statement::Empty);

        Ok(ast)
    }

    fn statement(&mut self) -> Result<Statement, ConstantError> {
        if let Some(o) = SINGLE_OPERATIONS.get(&self.current_token.token_type) {
            self.next();
            Ok(Statement::SingleOperation(*o))
        } else if let Some(o) = DOUBLE_OPERATIONS.get(&self.current_token.token_type) {
            self.next();
            Ok(Statement::DoubleOperation(*o))
        } else if matches!(
            self.current_token.token_type,
            TokenType::Number | TokenType::Bool | TokenType::String
        ) {
            let val = self.current_token.literal.clone().unwrap();
            self.next();
            Ok(Statement::Push(Value::Literal(val)))
        } else if self.check_token(TokenType::Bind) {
            self.match_token(TokenType::Bind)?;
            let ident = self.match_token(TokenType::Ident)?;
            Ok(Statement::Bind(ident.lexeme))
        } else if self.check_token(TokenType::Ident) {
            let tok = self.match_token(TokenType::Ident)?;
            Ok(Statement::Push(Value::Ident(tok.lexeme.clone())))
        } else if self.check_token(TokenType::If) {
            self.match_token(TokenType::If)?;
            let mut conditions = Vec::new();
            while !self.check_token(TokenType::Do) {
                match self.statement() {
                    Ok(statement) => conditions.push(statement),
                    Err(_) if self.check_token(TokenType::EOF) => {
                        return Err(ConstantError::NonMatchingToken(
                            TokenType::EOF,
                            vec![TokenType::Do],
                        ))
                    }
                    Err(e) => return Err(e),
                }
            }
            self.match_token(TokenType::Do)?;
            let mut statements = Vec::new();
            while !self.check_token(TokenType::EndIf) && !self.check_token(TokenType::Else) {
                match self.statement() {
                    Ok(statement) => statements.push(statement),
                    Err(_) if self.check_token(TokenType::EOF) => {
                        return Err(ConstantError::NonMatchingToken(
                            TokenType::EOF,
                            vec![TokenType::EndIf],
                        ))
                    }
                    Err(e) => return Err(e),
                }
            }
            let else_statements = if self.check_token(TokenType::Else) {
                self.next();
                let mut s = Vec::new();
                while !self.check_token(TokenType::EndIf) {
                    s.push(self.statement()?);
                    if self.previous_token.token_type == TokenType::EndIf {
                        break;
                    }
                }
                if self.previous_token.token_type != TokenType::EndIf {
                    self.next();
                }
                Some(s)
            } else {
                self.next();
                None
            };
            Ok(Statement::If(conditions, statements, else_statements))
        } else if self.check_token(TokenType::While) {
            self.match_token(TokenType::While)?;
            let mut conditions = Vec::new();
            while !self.check_token(TokenType::Do) {
                match self.statement() {
                    Ok(statement) => conditions.push(statement),
                    Err(_) if self.check_token(TokenType::EOF) => {
                        return Err(ConstantError::NonMatchingToken(
                            TokenType::EOF,
                            vec![TokenType::Do],
                        ))
                    }
                    Err(e) => return Err(e),
                }
            }
            self.match_token(TokenType::Do)?;
            let mut statements = Vec::new();
            while !self.check_token(TokenType::EndWhile) {
                match self.statement() {
                    Ok(statement) => statements.push(statement),
                    Err(_) if self.check_token(TokenType::EOF) => {
                        return Err(ConstantError::NonMatchingToken(
                            TokenType::EOF,
                            vec![TokenType::EndWhile],
                        ))
                    }
                    Err(e) => return Err(e),
                }
            }
            self.match_token(TokenType::EndWhile)?;
            Ok(Statement::While(conditions, statements))
        } else {
            Err(ConstantError::NonMatchingToken(
                self.current_token.token_type,
                vec![
                    SINGLE_OPERATIONS
                        .keys()
                        .map(|k| *k)
                        .collect::<Vec<TokenType>>(),
                    DOUBLE_OPERATIONS
                        .keys()
                        .map(|k| *k)
                        .collect::<Vec<TokenType>>(),
                    vec![
                        TokenType::Number,
                        TokenType::Bool,
                        TokenType::String,
                        TokenType::Bind,
                        TokenType::Ident,
                        TokenType::If,
                    ],
                ]
                .concat(),
            ))
        }
    }
}
