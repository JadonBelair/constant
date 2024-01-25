use anyhow::Result;

use error::ConstantError;
use interpreter::Interpreter;
use lexer::Lexer;

mod error;
mod interpreter;
mod lexer;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(ConstantError::NoSourceFile.into());
    }

    let file_contents = if let Ok(c) = std::fs::read_to_string(&args[1]) {
        c
    } else {
        return Err(ConstantError::SourceFileNotFound.into());
    };

    let tokens = Lexer::new(&file_contents).tokenize()?;
    Ok(Interpreter::new(&tokens).interpret()?)
}
