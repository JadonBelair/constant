use std::io::Write;

use crate::{
    error::ConstantError,
    lexer::{Lexer, Literal}, parser::{Statement, Value, SingleOpType, DoubleOpType, Parser},
};

pub struct Interpreter {
    stack: Vec<Literal>,
    program: Vec<Statement>,
    current_statement: Statement,
    current_pos: usize,
}

impl Interpreter {
    pub fn new(program: Vec<Statement>) -> Self {
        let mut program = program;
        if program.last() != Some(&Statement::Empty) {
            program.push(Statement::Empty);
        }
        let current_statement = program[0].clone();

        Self {
            stack: Vec::new(),
            program,
            current_statement,
            current_pos: 0,
        }
    }

    fn next(&mut self) {
        if let Some(t) = self.program.get(self.current_pos + 1) {
            self.current_pos += 1;
            self.current_statement = t.clone();
        }
    }

    pub fn interpret(&mut self) -> Result<(), ConstantError> {
        while self.current_statement != Statement::Empty {
            match &self.current_statement {
                Statement::Push(Value::Literal(l)) => self.stack.push(l.clone()),
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
                        // doens't really matter for interpreted mode
                        // but it's a nice feature to have in the REPL
                        self.stack.push(second);
                        return Err(ConstantError::InvalidStackAmount(String::from(action), 2));
                    };

                    if o == &DoubleOpType::Swap {
                        self.stack.push(first.clone());
                    }

                    let res = match o {
                        DoubleOpType::Add => first.clone() + second.clone(),
                        DoubleOpType::Sub => first.clone() - second.clone(),
                        DoubleOpType::Mul => first.clone() * second.clone(),
                        DoubleOpType::Div => first.clone() / second.clone(),
                        DoubleOpType::Swap => Ok(second.clone()),
                        DoubleOpType::GT => Ok(Literal::Bool(first.clone() > second.clone())),
                        DoubleOpType::GTEq => Ok(Literal::Bool(first.clone() >= second.clone())),
                        DoubleOpType::LT => Ok(Literal::Bool(first.clone() < second.clone())),
                        DoubleOpType::LTEq => Ok(Literal::Bool(first.clone() <= second.clone())),
                        DoubleOpType::Eq => Ok(Literal::Bool(first.clone() == second.clone())),
                        DoubleOpType::NotEq => Ok(Literal::Bool(first.clone() != second.clone())),
                    };

                    match res {
                        Ok(v) => self.stack.push(v),
                        Err(e) => {
                            self.stack.push(first);
                            self.stack.push(second);
                            return Err(e);
                        }
                    }
                }
                Statement::Empty => (),
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

            let tokens = match Lexer::new(&code).tokenize() {
                Ok(tokens) => tokens,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };
            let ast = match Parser::new(tokens).parse() {
                Ok(a) => a,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };
            
            self.program = ast;
            self.current_statement = self.program[0].clone();
            self.current_pos = 0;

            if let Err(e) = self.interpret() {
                println!("{e}");
            }
        }
    }
}
