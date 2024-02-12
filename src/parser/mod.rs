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
        h.insert(TokenType::Percent, DoubleOpType::Mod);
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

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current_token: Token,
    current_pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            current_token: tokens[0].clone(),
            current_pos: 0,
        }
    }

    fn next(&mut self) {
        if let Some(t) = self.tokens.get(self.current_pos + 1) {
            self.current_pos += 1;
            self.current_token = t.clone();
        }
    }

    fn check_token(&self, token: TokenType) -> bool {
        self.current_token.token_type == token
    }

    fn match_token(&mut self, token: TokenType) -> Result<Token, ConstantError> {
        if self.check_token(token) {
            let tok = self.current_token.clone();
            self.next();
            Ok(tok)
        } else {
            Err(ConstantError::UnexpectedToken(
                self.current_token.token_type,
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
            Ok(Statement::Push(Value::Ident(tok.lexeme)))
        } else if self.check_token(TokenType::If) {
            self.match_token(TokenType::If)?;
            let (conditions, statements) = self.if_block()?;

            let mut elifs = Vec::new();
            while !self.check_token(TokenType::Else) && !self.check_token(TokenType::End) {
                self.match_token(TokenType::Elif)?;
                elifs.push(self.if_block()?);
            }

            let mut else_statements = Vec::new();
            if self.check_token(TokenType::Else) {
                self.match_token(TokenType::Else)?;
                self.match_token(TokenType::Do)?;

                else_statements = self.get_statements_till(vec![TokenType::End])?;
            }
            self.match_token(TokenType::End)?;

            Ok(Statement::If(
                conditions,
                statements,
                elifs,
                else_statements,
            ))
        } else if self.check_token(TokenType::While) {
            self.match_token(TokenType::While)?;
            let conditions = self.get_statements_till(vec![TokenType::Do])?;

            self.match_token(TokenType::Do)?;

            let statements = self.get_statements_till(vec![TokenType::End])?;
            self.match_token(TokenType::End)?;

            Ok(Statement::While(conditions, statements))
        } else if self.check_token(TokenType::Proc) {
            self.match_token(TokenType::Proc)?;
            let ident = self.match_token(TokenType::Ident)?;
            self.match_token(TokenType::Do)?;
            let statements = self.get_statements_till(vec![TokenType::End])?;
            self.match_token(TokenType::End)?;
            Ok(Statement::Procedure(ident.lexeme, statements))
        } else if self.check_token(TokenType::Call) {
            self.match_token(TokenType::Call)?;
            let ident = self.match_token(TokenType::Ident)?;
            Ok(Statement::Call(ident.lexeme))
        } else {
            Err(ConstantError::UnexpectedToken(
                self.current_token.token_type,
            ))
        }
    }

    // gets the conditions and statements ran in an if block
    // but doesnt consume the ending elif, else, or endif
    fn if_block(&mut self) -> Result<(Vec<Statement>, Vec<Statement>), ConstantError> {
        let conditions = self.get_statements_till(vec![TokenType::Do])?;

        self.match_token(TokenType::Do)?;

        let statements =
            self.get_statements_till(vec![TokenType::Elif, TokenType::Else, TokenType::End])?;

        Ok((conditions, statements))
    }

    fn get_statements_till(
        &mut self,
        tokens: Vec<TokenType>,
    ) -> Result<Vec<Statement>, ConstantError> {
        let mut statements = Vec::new();
        while !tokens.contains(&self.current_token.token_type) {
            match self.statement() {
                Ok(statement) => statements.push(statement),
                Err(_) => {
                    return Err(ConstantError::UnexpectedToken(
                        self.current_token.token_type,
                    ))
                }
            }
        }

        Ok(statements)
    }
}
