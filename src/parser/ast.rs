use crate::lexer::Literal;

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Push(Value),
    DoubleOperation(DoubleOpType),
    SingleOperation(SingleOpType),
    Bind(String),
    If(
        Vec<Statement>,
        Vec<Statement>,
        Vec<(Vec<Statement>, Vec<Statement>)>,
        Vec<Statement>,
    ),
    While(Vec<Statement>, Vec<Statement>),
    Procedure(String, Vec<Statement>),
    Call(String),
    Empty,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DoubleOpType {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    GT,
    GTEq,
    LT,
    LTEq,
    Eq,
    NotEq,
    And,
    Or,
    Swap,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SingleOpType {
    Print,
    Dup,
    Drop,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Literal(Literal),
    Ident(String),
}
