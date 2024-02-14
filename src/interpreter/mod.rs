use std::{collections::HashMap, io::Write};

use crate::{
    error::ConstantError,
    lexer::{Lexer, Literal},
    parser::{DoubleOpType, Parser, SingleOpType, Statement, Value},
};

pub struct Interpreter {
    stack: Vec<Literal>,
    program: Vec<Statement>,
    idents: HashMap<String, Literal>,
    procs: HashMap<String, Vec<Statement>>,
}

impl Interpreter {
    pub fn new(program: Vec<Statement>) -> Self {
        Self {
            stack: Vec::new(),
            program,
            idents: HashMap::new(),
            procs: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<(), ConstantError> {
        for statement in self.program.clone() {
            self.interpret_statement(&statement)?;
        }
        Ok(())
    }

    fn interpret_statement(&mut self, statement: &Statement) -> Result<(), ConstantError> {
        match statement {
            Statement::Push(Value::Literal(l)) => self.stack.push(l.clone()),
            Statement::Push(Value::Ident(i)) => {
                if let Some(v) = self.idents.get(i) {
                    self.stack.push(v.clone());
                } else {
                    return Err(ConstantError::IdentDoesNotExist(i.into()));
                }
            }
            Statement::SingleOperation(o) => {
                let action = match o {
                    SingleOpType::Print => "Printing",
                    SingleOpType::Dup => "Duping",
                    SingleOpType::Drop => "Dropping",
                };

                let val = if let Some(val) = self.stack.pop() {
                    val
                } else {
                    return Err(ConstantError::InvalidStackAmount(String::from(action), 1));
                };

                match o {
                    SingleOpType::Print => println!("{val}"),
                    SingleOpType::Dup => {
                        self.stack.push(val.clone());
                        self.stack.push(val);
                    }
                    SingleOpType::Drop => (),
                }
            }
            Statement::DoubleOperation(o) => {
                let action = match o {
                    DoubleOpType::Add => "Addition",
                    DoubleOpType::Sub => "Subtraction",
                    DoubleOpType::Mul => "Multiplication",
                    DoubleOpType::Div => "Division",
                    DoubleOpType::Swap => "Swapping",
                    DoubleOpType::Mod => "Modulo",
                    _ => "Comparison",
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
                    // doesn't really matter for interpreted mode
                    // but it's a nice feature to have in the REPL
                    self.stack.push(second);
                    return Err(ConstantError::InvalidStackAmount(String::from(action), 2));
                };

                if *o == DoubleOpType::Swap {
                    self.stack.push(second.clone());
                }

                let res = |x: Literal, y: Literal| match o {
                    DoubleOpType::Add => x + y,
                    DoubleOpType::Sub => x - y,
                    DoubleOpType::Mul => x * y,
                    DoubleOpType::Div => x / y,
                    DoubleOpType::Mod => x % y,
                    DoubleOpType::Swap => Ok(x),
                    DoubleOpType::GT => Ok(Literal::Bool(x > y)),
                    DoubleOpType::GTEq => Ok(Literal::Bool(x >= y)),
                    DoubleOpType::LT => Ok(Literal::Bool(x < y)),
                    DoubleOpType::LTEq => Ok(Literal::Bool(x <= y)),
                    DoubleOpType::Eq => Ok(Literal::Bool(x == y)),
                    DoubleOpType::NotEq => Ok(Literal::Bool(x != y)),
                    DoubleOpType::And | DoubleOpType::Or => {
                        match (x, y) {
                            (Literal::Bool(a), Literal::Bool(b)) => {
                                match o {
                                    DoubleOpType::And => Ok(Literal::Bool(a && b)),
                                    DoubleOpType::Or => Ok(Literal::Bool(a || b)),
                                    _ => unreachable!()
                                }
                            }
                            _ => return Err(ConstantError::InvalidOperation("Logical operations can only be performed on bools".into()))
                        }
                    }
                };

                match res(first.clone(), second.clone()) {
                    Ok(v) => self.stack.push(v),
                    Err(e) => {
                        self.stack.push(first);
                        self.stack.push(second);
                        return Err(e);
                    }
                }
            }
            Statement::Bind(ident) => {
                let val = if let Some(val) = self.stack.pop() {
                    val
                } else {
                    return Err(ConstantError::InvalidStackAmount("Binding".into(), 1));
                };

                self.idents.insert(ident.into(), val);

                if self.procs.contains_key(ident) {
                    // ensures that "ident" is either a literal
                    // or a procedure but not both
                    self.procs.remove(ident);
                }
            }
            Statement::If(conditions, statements, elifs, else_statements) => {
                for statement in conditions {
                    self.interpret_statement(statement)?;
                }

                let val = if let Some(Literal::Bool(b)) = self.stack.pop() {
                    b
                } else {
                    return Err(ConstantError::InvalidOperation(
                        "If statement expects boolean value on top of stack".into(),
                    ));
                };

                if val {
                    for statement in statements {
                        self.interpret_statement(statement)?;
                    }
                } else {
                    let mut do_else = true;
                    for (elif_conditions, elif_statements) in elifs {
                        for elif_condition in elif_conditions {
                            self.interpret_statement(elif_condition)?;
                        }
                        let val = if let Some(Literal::Bool(b)) = self.stack.pop() {
                            b
                        } else {
                            return Err(ConstantError::InvalidOperation(
                                "If statement expects boolean value on top of stack".into(),
                            ));
                        };

                        if val {
                            do_else = false;
                            for elif_statement in elif_statements {
                                self.interpret_statement(elif_statement)?;
                            }
                            break;
                        }
                    }
                    if do_else {
                        for else_statement in else_statements {
                            self.interpret_statement(else_statement)?;
                        }
                    }
                }
            }
            Statement::While(conditions, statements) => loop {
                for statement in conditions {
                    self.interpret_statement(statement)?;
                }
                let val = if let Some(Literal::Bool(b)) = self.stack.pop() {
                    b
                } else {
                    return Err(ConstantError::InvalidOperation(
                        "While statement expects boolean value on top of stack".into(),
                    ));
                };

                if val {
                    for statement in statements {
                        self.interpret_statement(statement)?;
                    }
                } else {
                    break;
                }
            },
            Statement::Procedure(ident, statements) => {
                self.procs.insert(ident.into(), statements.to_vec());

                if self.idents.contains_key(ident) {
                    self.idents.remove(ident);
                }
            }
            Statement::Call(ident) => {
                if let Some(statements) = self.procs.get(ident).cloned() {
                    for statement in statements {
                        self.interpret_statement(&statement)?;
                    }
                } else {
                    return Err(ConstantError::ProcDoesNotExist(ident.into()));
                }
            }
            Statement::Empty => (),
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

            let tokens = match Lexer::new(&code).tokenize() {
                Ok(tokens) => tokens,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };
            let ast = match Parser::new(&tokens).parse() {
                Ok(a) => a,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };

            self.program = ast;

            if let Err(e) = self.interpret() {
                println!("{e}");
            }
        }
    }
}
