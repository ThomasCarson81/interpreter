use std::{collections::HashMap, env, time};
use syntax::{Expression, Program, Statement, TokenType, Value};
mod builtins;
#[derive(Debug)]
enum InterpreterError {
    CouldNotReadFile(String),
}
#[derive(Debug)]
enum RuntimeError {
    UndefinedVariable(String),
    UndefinedFunction(String),
    OutOfScopes,
    TypeError(String),
    BinOpTypeError(Value, TokenType, Value),
    UnknownOperator(TokenType),
    DivideByZero,
    ArgumentCountMismatch(String, usize, usize),
    IllegalControlFlow(ControlFlow),
}

#[derive(Debug)]
enum ControlFlow {
    Normal,
    Return(Value),
    Break,
    Continue,
}

type BuiltInFunction = fn(Vec<Value>, &mut Environment) -> Result<Value, RuntimeError>;

#[derive(Clone)]
enum Function {
    UserDefined {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    BuiltIn {
        function: BuiltInFunction,
    },
}
impl Function {
    fn call(
        &self,
        arguments: Vec<Value>,
        environment: &mut Environment,
    ) -> Result<Value, RuntimeError> {
        match self {
            Function::UserDefined {
                name,
                parameters,
                body,
            } => {
                let num_params = parameters.len();
                let num_args = arguments.len();
                if num_params != num_args {
                    return Err(RuntimeError::ArgumentCountMismatch(
                        name.clone(),
                        num_params,
                        num_args,
                    ));
                }
                environment.push_scope();
                for (parameter, argument) in parameters.iter().zip(arguments) {
                    environment.define(parameter.clone(), argument)?;
                }
                match execute_block(&body, environment)? {
                    ControlFlow::Normal => {}
                    ControlFlow::Return(value) => {
                        environment.pop_scope();
                        return Ok(value);
                    }
                    ControlFlow::Break => {
                        environment.pop_scope();
                        return Err(RuntimeError::IllegalControlFlow(ControlFlow::Break));
                    }
                    ControlFlow::Continue => {
                        environment.pop_scope();
                        return Err(RuntimeError::IllegalControlFlow(ControlFlow::Continue));
                    }
                }
                environment.pop_scope();
                Ok(Value::Unit)
            }
            Function::BuiltIn { function, .. } => function(arguments, environment),
        }
    }
}

struct Environment {
    scopes: Vec<HashMap<String, Value>>,
    functions: HashMap<String, Function>,
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

    fn define_func(&mut self, name: String, value: Function) {
        self.functions.insert(name, value);
    }

    fn define_builtin_func(&mut self, name: String, function: BuiltInFunction) {
        self.functions
            .insert(name.clone(), Function::BuiltIn { function });
    }

    fn call_func(&mut self, name: String, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        let func = self
            .functions
            .get(&name)
            .cloned()
            .ok_or(RuntimeError::UndefinedFunction(name))?;
        func.call(arguments, self)
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
    let mut environment = Environment {
        scopes: Vec::new(),
        functions: HashMap::new(),
    };
    builtins::register_all(&mut environment);
    environment.push_scope();
    let start_time = time::Instant::now();
    let result = execute_program(&program, &mut environment);
    let end_time = time::Instant::now();
    let total_time = end_time - start_time;
    match result {
        Ok(_) => {
            println!("\nProgram finished executing in {:?}", total_time);
        }
        Err(e) => match e {
            RuntimeError::UndefinedVariable(v) => println!("Undefined variable: {v}"),
            RuntimeError::UndefinedFunction(f) => println!("Undefined function: {f}"),
            RuntimeError::OutOfScopes => println!("Interpreter ran out of scopes."),
            RuntimeError::TypeError(msg) => println!("TypeError: {msg}"),
            RuntimeError::BinOpTypeError(left, op, right) => println!(
                "TypeError: operator: '{op}' is incompatible between types '{:?}' and '{:?}'",
                left, right
            ),
            RuntimeError::UnknownOperator(op) => println!("Unknown operator: {op}"),
            RuntimeError::DivideByZero => println!("Cannot divide by zero."),
            RuntimeError::ArgumentCountMismatch(name, num_param, num_arg) => println!(
                "Error: Function '{name}' takes {num_param} arguments, but '{num_arg}' were provided."
            ),
            RuntimeError::IllegalControlFlow(control_flow) => match control_flow {
                ControlFlow::Normal => println!("Somehow normal control flow is illegal."),
                ControlFlow::Return(_) => {
                    println!("Return statements may only be used inside functions")
                }
                ControlFlow::Break => println!("Break statements may only be used inside loops."),
                ControlFlow::Continue => {
                    println!("Continue statements may only be used inside loops.")
                }
            },
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
) -> Result<ControlFlow, RuntimeError> {
    environment.push_scope();

    for statement in block {
        match execute_statement(&statement, environment)? {
            ControlFlow::Normal => {}
            flow => {
                environment.pop_scope();
                return Ok(flow);
            }
        }
    }

    environment.pop_scope();

    Ok(ControlFlow::Normal)
}
fn execute_statement(
    statement: &Statement,
    environment: &mut Environment,
) -> Result<ControlFlow, RuntimeError> {
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
            let condition_result = evaluate_expression(condition, environment)?;
            let exec = match condition_result {
                Value::Bool(b) => Ok(b),
                _ => Err(RuntimeError::TypeError(String::from(
                    "expressions in 'if' statements must evaluate to a boolean.",
                ))),
            }?;
            return if exec {
                execute_block(body, environment)
            } else if let Some(else_block) = else_block {
                execute_block(else_block, environment)
            } else {
                Ok(ControlFlow::Normal)
            };
        }
        Statement::While { condition, body } => {
            let mut repeat = match evaluate_expression(condition, environment)? {
                Value::Bool(b) => Ok(b),
                _ => Err(RuntimeError::TypeError(String::from(
                    "expressions in 'if' statements must evaluate to a boolean.",
                ))),
            }?;
            while repeat {
                match execute_block(body, environment)? {
                    ControlFlow::Normal | ControlFlow::Continue => {}
                    ControlFlow::Break => break,
                    ControlFlow::Return(value) => return Ok(ControlFlow::Return(value)),
                }
                repeat = match evaluate_expression(condition, environment)? {
                    Value::Bool(b) => Ok(b),
                    _ => Err(RuntimeError::TypeError(String::from(
                        "conditions 'while' statements must evaluate to a boolean.",
                    ))),
                }?;
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
                    let again = match evaluate_expression(condition, environment)? {
                        Value::Bool(b) => Ok(b),
                        _ => Err(RuntimeError::TypeError(String::from(
                            "conditions 'for' statements must evaluate to a boolean.",
                        ))),
                    }?;
                    if !again {
                        break;
                    }
                }

                match execute_block(body, environment)? {
                    ControlFlow::Normal | ControlFlow::Continue => {}
                    ControlFlow::Return(value) => {
                        environment.pop_scope();
                        return Ok(ControlFlow::Return(value));
                    }
                    ControlFlow::Break => break,
                }

                if let Some(increment) = increment {
                    execute_statement(increment, environment)?;
                }
            }
            environment.pop_scope();
        }
        Statement::Scope { body } => {
            return execute_block(body, environment);
        }
        Statement::FunctionDefinition {
            identifier,
            parameters,
            body,
        } => {
            let func = Function::UserDefined {
                name: identifier.clone(),
                parameters: parameters.clone(),
                body: body.clone(),
            };
            environment.define_func(identifier.clone(), func);
        }
        Statement::ExpressionStatement { expression } => {
            evaluate_expression(expression, environment)?;
        }
        Statement::Return { value } => {
            let value = match value {
                Some(expression) => evaluate_expression(expression, environment)?,
                None => Value::Unit,
            };

            return Ok(ControlFlow::Return(value));
        }
        Statement::Break => return Ok(ControlFlow::Break),
        Statement::Continue => return Ok(ControlFlow::Continue),
        Statement::Pass => (),
    }

    Ok(ControlFlow::Normal)
}

fn evaluate_expression(
    expression: &Expression,
    environment: &mut Environment,
) -> Result<Value, RuntimeError> {
    match expression {
        Expression::Literal(val) => Ok(val.clone()),
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
        Expression::FunctionCall {
            identifier,
            arguments,
        } => {
            let args = arguments
                .iter()
                .map(|arg| evaluate_expression(arg, environment))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(environment.call_func(identifier.clone(), args)?)
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
        TokenType::Minus => evaluate_unary_minus(&evaluate_expression(operand, environment)?),
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
        (Value::Number(left), Value::Number(right)) => Ok(Value::Bool(left > right)),
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::GreaterThan,
            right,
        )),
    }
}

fn evaluate_greater_than_or_equal_to(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Bool(left >= right)),
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::GreaterThanOrEqualTo,
            right,
        )),
    }
}

fn evaluate_less_than(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Bool(left < right)),
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::LessThan,
            right,
        )),
    }
}

fn evaluate_less_than_or_equal_to(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Bool(left <= right)),
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::LessThanOrEqualTo,
            right,
        )),
    }
}

fn evaluate_equal_to(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Bool(left == right)),
        (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(left == right)),
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::EqualTo,
            right,
        )),
    }
}

fn evaluate_not_equal_to(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Ok(Value::Bool(left != right)),
        (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(left != right)),
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::NotEqualTo,
            right,
        )),
    }
}

fn evaluate_logical_and(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(*left && *right)),
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::LogicalAnd,
            right,
        )),
    }
}

fn evaluate_logical_or(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (&left, &right) {
        (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(*left || *right)),
        _ => Err(RuntimeError::BinOpTypeError(
            left,
            TokenType::LogicalOr,
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
    match operand {
        Value::Bool(b) => Ok(Value::Bool(!b)),
        _ => Err(RuntimeError::TypeError(String::from(
            "operator: '!' is only compatible with booleans.",
        ))),
    }
}

fn evaluate_unary_minus(operand: &Value) -> Result<Value, RuntimeError> {
    match operand {
        Value::Number(num) => Ok(Value::Number(-num)),
        _ => Err(RuntimeError::TypeError(String::from(
            "operator: '-' is only compatible with numbers.",
        ))),
    }
}
