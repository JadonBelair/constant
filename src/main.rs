use anyhow::Result;

use error::ConstantError;
use interpreter::Interpreter;
use lexer::Lexer;

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
        println!("{tokens:?}");
        Ok(Interpreter::new(tokens).interpret()?)
    } else {
        return Err(ConstantError::NoSourceFile.into());
    }
}
