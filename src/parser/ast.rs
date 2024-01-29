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
    Empty,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DoubleOpType {
    Add,
    Sub,
    Mul,
    Div,
    GT,
    GTEq,
    LT,
    LTEq,
    Eq,
    NotEq,
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
