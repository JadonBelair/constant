use error::ConstantError;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

mod error;
mod interpreter;
mod lexer;
mod parser;

fn main() -> Result<(), ConstantError> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        Ok(Interpreter::new(Vec::new()).repl())
    } else if args.len() == 2 {
        let file_contents = std::fs::read_to_string(&args[1])
            .map_err(|_| ConstantError::SourceFileNotFound(args[1].clone()))?;

        let tokens = Lexer::new(&file_contents).tokenize()?;
        let ast = Parser::new(&tokens).parse()?;

        Interpreter::new(ast).interpret()
    } else if args.len() > 2 {
        return Err(ConstantError::TooManyArgs);
    } else {
        return Err(ConstantError::NoSourceFile);
    }
}
