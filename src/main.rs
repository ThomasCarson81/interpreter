use std::{collections::HashMap, env};
use syntax::{Expression, Program, Statement, Token, TokenType};

#[derive(Debug)]
enum InterpreterError {
    CouldNotReadFile(String),
}
#[derive(Debug)]
enum RuntimeError {
    UndefinedVariable(String),
    OutOfScopes,
}

#[derive(Debug)]
enum Value {
    Number(f64),
}
impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Number(n) => *n > 0.0,
        }
    }
}

struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}
impl Environment {
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, name: String, value: Value) -> Result<(), RuntimeError> {
        self.scopes
            .last_mut()
            .ok_or(RuntimeError::OutOfScopes)?
            .insert(name, value);
        Ok(())
    }

    fn get(&self, name: &str) -> Option<&Value> {
        self.scopes.iter().rev().find_map(|scope| scope.get(name))
    }

    fn assign(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(RuntimeError::UndefinedVariable(name.to_string()))
    }
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
    let mut environment = Environment { scopes: Vec::new() };
    environment.push_scope();
    match execute_program(&program, &mut environment) {
        Ok(_) => {
            println!("Program executed!");
            println!("Global variables: ");
            for scope in environment.scopes {
                for (key, value) in scope.iter() {
                    println!("{} = {:?}", key, value)
                }
            }
        }
        Err(e) => {
            println!("Runtime Error: {:?}", e)
        }
    }
}
fn get_ast_from_file(file_path: &str) -> Result<Program, InterpreterError> {
    let ast = std::fs::read_to_string(&file_path)
        .map_err(|e| InterpreterError::CouldNotReadFile(e.to_string()))?;
    let program: Program = serde_json::from_str(&ast)
        .map_err(|e| InterpreterError::CouldNotReadFile(e.to_string()))?;
    Ok(program)
}

fn execute_program(program: &Program, environment: &mut Environment) -> Result<(), RuntimeError> {
    for statement in &program.statements {
        execute_statement(statement, environment)?;
    }
    Ok(())
}
fn execute_block(
    block: &Vec<Statement>,
    environment: &mut Environment,
) -> Result<(), RuntimeError> {
    environment.push_scope();

    for statement in block {
        execute_statement(&statement, environment)?;
    }

    environment.pop_scope();

    Ok(())
}
fn execute_statement(
    statement: &Statement,
    environment: &mut Environment,
) -> Result<(), RuntimeError> {
    match statement {
        Statement::Let {
            identifier,
            expression,
        } => {
            environment.define(
                identifier.clone(),
                evaluate_expression(&expression, &environment)?,
            )?;
        }
        Statement::Assignment {
            identifier,
            expression,
        } => {
            environment.assign(identifier, evaluate_expression(expression, environment)?)?;
        }
        Statement::If { condition, body } => {
            if evaluate_expression(condition, environment)?.is_truthy() {
                execute_block(body, environment)?;
            }
        }
        Statement::Else { body } => todo!(),
        Statement::While { condition, body } => todo!(),
        Statement::For {
            initialiser,
            condition,
            increment,
            body,
        } => todo!(),
        Statement::ExpressionStatement { expression } => {
            evaluate_expression(expression, environment)?;
        }
        Statement::Pass => (),
        _ => (),
    }

    Ok(())
}

fn evaluate_expression(
    expression: &Expression,
    environment: &Environment,
) -> Result<Value, RuntimeError> {
    Ok(Value::Number(0.0))
}
