use anyhow::Result;

use error::ConstantError;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

mod error;
mod interpreter;
mod lexer;
mod parser;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        Ok(Interpreter::new(Vec::new()).repl())
    } else if args.len() == 2 {
        let file_contents = if let Ok(c) = std::fs::read_to_string(&args[1]) {
            c
        } else {
            return Err(ConstantError::SourceFileNotFound(args[1].clone()).into());
        };

        let tokens = Lexer::new(&file_contents).tokenize()?;
        let ast = Parser::new(tokens).parse()?;
        Ok(Interpreter::new(ast).interpret()?)
    } else if args.len() > 2 {
        return Err(ConstantError::TooManyArgs.into());
    } else {
        return Err(ConstantError::NoSourceFile.into());
    }
}
