use std::env;
use syntax::{Expression, Program, Statement, Token, TokenType};

#[derive(Debug)]
enum InterpreterError {
    CouldNotReadFile(String),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a file to interpret.");
        return;
    }

    let program = match get_ast_from_file(&args[1]) {
        Ok(ast) => ast,
        Err(e) => {
            match e {
                InterpreterError::CouldNotReadFile(s) => {
                    println!("Error reading file: {}", s);
                }
            }
            std::process::exit(1);
        }
    };
}
fn get_ast_from_file(file_path: &str) -> Result<Program, InterpreterError> {
    let ast = std::fs::read_to_string(&file_path)
        .map_err(|e| InterpreterError::CouldNotReadFile(e.to_string()))?;
    let program: Program = serde_json::from_str(&ast)
        .map_err(|e| InterpreterError::CouldNotReadFile(e.to_string()))?;
    Ok(program)
}
