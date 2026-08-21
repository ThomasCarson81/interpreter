use std::{collections::HashMap, env};
use syntax::{Expression, Program, Statement, TokenType};

#[derive(Debug)]
enum InterpreterError {
    CouldNotReadFile(String),
}
#[derive(Debug)]
enum RuntimeError {
    UndefinedVariable(String),
    OutOfScopes,
    TypeError(String),
    BinOpTypeError(Value, TokenType, Value),
    UnknownOperator(TokenType),
    DivideByZero,
}

#[derive(Debug, Clone)]
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
        Err(e) => match e {
            RuntimeError::UndefinedVariable(v) => println!("Undefined variable: {v}"),
            RuntimeError::OutOfScopes => println!("Interpreter ran out of scopes."),
            RuntimeError::TypeError(msg) => println!("TypeError: {msg}"),
            RuntimeError::BinOpTypeError(left, op, right) => println!(
                "TypeError: operator: '{op}' is incompatible between types '{:?}' and '{:?}'",
                left, right
            ),
            RuntimeError::UnknownOperator(op) => println!("Unknown operator: {op}"),
            RuntimeError::DivideByZero => println!("Cannot divide by zero."),
        },
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
            let val = evaluate_expression(&expression, environment)?;
            environment.define(identifier.clone(), val)?;
        }
        Statement::Assignment {
            identifier,
            expression,
        } => {
            let val = evaluate_expression(expression, environment)?;
            environment.assign(identifier, val)?;
        }
        Statement::If {
            condition,
            body,
            else_block,
        } => {
            if evaluate_expression(condition, environment)?.is_truthy() {
                execute_block(body, environment)?;
            } else if let Some(else_block) = else_block {
                execute_block(else_block, environment)?;
            }
        }
        Statement::While { condition, body } => {
            while evaluate_expression(condition, environment)?.is_truthy() {
                execute_block(body, environment)?;
            }
        }
        Statement::For {
            initialiser,
            condition,
            increment,
            body,
        } => {
            environment.push_scope();
            if let Some(init) = initialiser {
                execute_statement(init, environment)?;
            }
            loop {
                if let Some(condition) = condition {
                    if !evaluate_expression(condition, environment)?.is_truthy() {
                        break;
                    }
                }

                execute_block(body, environment)?;

                if let Some(increment) = increment {
                    execute_statement(increment, environment)?
                };
            }
            environment.pop_scope();
        }
        Statement::ExpressionStatement { expression } => {
            evaluate_expression(expression, environment)?;
        }
        Statement::Pass => (),
    }

    Ok(())
}

fn evaluate_expression(
    expression: &Expression,
    environment: &mut Environment,
) -> Result<Value, RuntimeError> {
    match expression {
        Expression::Number(num) => Ok(Value::Number(*num)),
        Expression::Identifier(name) => environment
            .get(name)
            .cloned()
            .ok_or(RuntimeError::UndefinedVariable(name.clone())),
        Expression::BinaryOp {
            left,
            operator,
            right,
        } => {
            let left = evaluate_expression(left, environment)?;
            let right = evaluate_expression(right, environment)?;
            evaluate_binary_operator(left, operator, right)
        }
        Expression::UnaryOp { operator, operand } => {
            evaluate_unary_operator(operand, operator, environment)
        }
    }
}
fn evaluate_binary_operator(
    left: Value,
    operator: &TokenType,
    right: Value,
) -> Result<Value, RuntimeError> {
    match operator {
        TokenType::Plus => evaluate_plus(left, right),
        TokenType::Minus => evaluate_minus(left, right),
        TokenType::Asterisk => evaluate_multiply(left, right),
        TokenType::Slash => evaluate_divide(left, right),
        TokenType::GreaterThan => evaluate_greater_than(left, right),
        TokenType::LessThan => evaluate_less_than(left, right),
        TokenType::GreaterThanOrEqualTo => evaluate_greater_than_or_equal_to(left, right),
        TokenType::LessThanOrEqualTo => evaluate_less_than_or_equal_to(left, right),
        TokenType::EqualTo => evaluate_equal_to(left, right),
        TokenType::NotEqualTo => evaluate_not_equal_to(left, right),
        TokenType::LogicalOr => evaluate_logical_or(left, right),
        TokenType::LogicalAnd => evaluate_logical_and(left, right),
        _ => Err(RuntimeError::UnknownOperator(*operator)),
    }
}

fn evaluate_unary_operator(
    operand: &Expression,
    operator: &TokenType,
    environment: &mut Environment,
) -> Result<Value, RuntimeError> {
    match operator {
        TokenType::Increment => evaluate_increment(operand, environment),
        TokenType::Decrement => evaluate_decrement(operand, environment),
        TokenType::Not => evaluate_not_operator(&evaluate_expression(operand, environment)?),
        _ => Err(RuntimeError::UnknownOperator(*operator)),
    }
}

fn evaluate_plus(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
        _ => Err(RuntimeError::BinOpTypeError(left, TokenType::Plus, right)),
    }
}

fn evaluate_minus(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left - right)),
        _ => Err(RuntimeError::BinOpTypeError(left, TokenType::Minus, right)),
    }
}

fn evaluate_multiply(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left * right)),
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::Asterisk,
            right,
        )),
    }
}

fn evaluate_divide(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => {
            if *right == 0.0 {
                return Err(RuntimeError::DivideByZero);
            }
            Ok(Value::Number(left / right))
        }
        _ => Err(RuntimeError::BinOpTypeError(left, TokenType::Slash, right)),
    }
}

fn evaluate_greater_than(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => {
            if left > right {
                return Ok(Value::Number(1.0));
            }
            Ok(Value::Number(0.0))
        }
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::GreaterThan,
            right,
        )),
    }
}

fn evaluate_greater_than_or_equal_to(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => {
            if left >= right {
                return Ok(Value::Number(1.0));
            }
            Ok(Value::Number(0.0))
        }
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::GreaterThanOrEqualTo,
            right,
        )),
    }
}

fn evaluate_less_than(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => {
            if left < right {
                return Ok(Value::Number(1.0));
            }
            Ok(Value::Number(0.0))
        }
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::LessThan,
            right,
        )),
    }
}

fn evaluate_less_than_or_equal_to(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => {
            if left <= right {
                return Ok(Value::Number(1.0));
            }
            Ok(Value::Number(0.0))
        }
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::LessThanOrEqualTo,
            right,
        )),
    }
}

fn evaluate_equal_to(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => {
            if left == right {
                return Ok(Value::Number(1.0));
            }
            Ok(Value::Number(0.0))
        }
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::EqualTo,
            right,
        )),
    }
}

fn evaluate_not_equal_to(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => {
            if left != right {
                return Ok(Value::Number(1.0));
            }
            Ok(Value::Number(0.0))
        }
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::NotEqualTo,
            right,
        )),
    }
}

fn evaluate_logical_and(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(_), Value::Number(_)) => {
            if left.is_truthy() && right.is_truthy() {
                return Ok(Value::Number(1.0));
            }
            Ok(Value::Number(0.0))
        }
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::NotEqualTo,
            right,
        )),
    }
}

fn evaluate_logical_or(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(_), Value::Number(_)) => {
            if left.is_truthy() || right.is_truthy() {
                return Ok(Value::Number(1.0));
            }
            Ok(Value::Number(0.0))
        }
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::NotEqualTo,
            right,
        )),
    }
}

fn evaluate_increment(
    operand: &Expression,
    environment: &mut Environment,
) -> Result<Value, RuntimeError> {
    match operand {
        Expression::Identifier(name) => {
            let num = environment
                .get(name)
                .ok_or(RuntimeError::UndefinedVariable(name.clone()))?;
            let Value::Number(n) = num else {
                return Err(RuntimeError::TypeError(String::from(
                    "operator: '++' is only compatible with numbers.",
                )));
            };
            let new_num = Value::Number(n + 1.0);
            environment.assign(name, new_num.clone())?;
            Ok(new_num)
        }
        _ => Err(RuntimeError::TypeError(String::from(
            "operator: '++' is only compatible with numbers.",
        ))),
    }
}

fn evaluate_decrement(
    operand: &Expression,
    environment: &mut Environment,
) -> Result<Value, RuntimeError> {
    match operand {
        Expression::Identifier(name) => {
            let num = environment
                .get(name)
                .ok_or(RuntimeError::UndefinedVariable(name.clone()))?;
            let Value::Number(n) = num else {
                return Err(RuntimeError::TypeError(String::from(
                    "operator: '--' is only compatible with numbers.",
                )));
            };
            let new_num = Value::Number(n - 1.0);
            environment.assign(name, new_num.clone())?;
            Ok(new_num)
        }
        _ => Err(RuntimeError::TypeError(String::from(
            "operator: '--' is only compatible with numbers.",
        ))),
    }
}

fn evaluate_not_operator(operand: &Value) -> Result<Value, RuntimeError> {
    if !operand.is_truthy() {
        Ok(Value::Number(1.0))
    } else {
        Ok(Value::Number(0.0))
    }
}
