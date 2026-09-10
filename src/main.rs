use std::{collections::HashMap, env, time};
use syntax::{
    ast::{Expression, Statement},
    functions::{CheckedProgram, Function, FunctionImplementation, FunctionTable},
    types::Value,
};
mod builtins;
mod operations;

#[derive(Debug)]
enum InterpreterError {
    CouldNotReadFile(String),
}

#[derive(Debug)]
enum ControlFlow {
    Normal,
    Return(Value),
    Break,
    Continue,
}

struct Environment {
    scopes: Vec<HashMap<String, Value>>,
    function_table: FunctionTable,
}
impl Environment {
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, name: String, value: Value) {
        let last_scope = self
            .scopes
            .last_mut()
            .expect("name resolver missed scope error");
        last_scope.insert(name, value);
    }

    fn get(&self, name: &str) -> &Value {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
            .expect("name resolver missed undeclared variable in get")
    }

    fn assign(&mut self, name: &str, value: Value) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return;
            }
        }
        unreachable!("name resolver missed undeclared variable in assignment")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a file to interpret.");
        return;
    }

    let checked_program = match get_checked_program_from_file(&args[1]) {
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
        function_table: checked_program.function_table.clone(),
    };
    environment.push_scope();
    let start_time = time::Instant::now();
    execute_checked_program(&checked_program, &mut environment);
    let end_time = time::Instant::now();
    let total_time = end_time - start_time;

    println!("\nProgram finished executing in {:?}", total_time);
}

fn get_checked_program_from_file(file_path: &str) -> Result<CheckedProgram, InterpreterError> {
    let ast = std::fs::read_to_string(file_path)
        .map_err(|e| InterpreterError::CouldNotReadFile(e.to_string()))?;
    let program: CheckedProgram = serde_json::from_str(&ast)
        .map_err(|e| InterpreterError::CouldNotReadFile(e.to_string()))?;
    Ok(program)
}

fn execute_checked_program(program: &CheckedProgram, environment: &mut Environment) {
    for statement in &program.globals {
        match execute_statement(statement, environment) {
            ControlFlow::Normal => {}
            _ => unreachable!("checked globals may not contain control-flow statements"),
        }
    }

    let main = environment
        .function_table
        .get(&program.main_function_id)
        .expect("checked program main function is missing from the function table")
        .clone();
    call_function(&main, Vec::new(), environment);
}

fn execute_block(block: &Vec<Statement>, environment: &mut Environment) -> ControlFlow {
    environment.push_scope();

    for statement in block {
        match execute_statement(&statement, environment) {
            ControlFlow::Normal => {}
            flow => {
                environment.pop_scope();
                return flow;
            }
        }
    }

    environment.pop_scope();

    ControlFlow::Normal
}

fn execute_statement(statement: &Statement, environment: &mut Environment) -> ControlFlow {
    match statement {
        Statement::Let {
            identifier,
            expression,
            mutable: _, // mutability and types are checked before runtime
            var_type: _,
        } => {
            let val = evaluate_expression(&expression, environment);
            environment.define(identifier.clone(), val);
        }
        Statement::Assignment {
            identifier,
            expression,
        } => {
            let val = evaluate_expression(expression, environment);
            environment.assign(identifier, val);
        }
        Statement::If {
            condition,
            body,
            else_block,
        } => {
            let condition_result = evaluate_expression(condition, environment);
            let exec = match condition_result {
                Value::Bool(b) => b,
                _ => unreachable!("type checker missed if statement non bool expression"),
            };
            return if exec {
                execute_block(body, environment)
            } else if let Some(else_block) = else_block {
                execute_block(else_block, environment)
            } else {
                ControlFlow::Normal
            };
        }
        Statement::While { condition, body } => {
            let mut repeat = match evaluate_expression(condition, environment) {
                Value::Bool(b) => b,
                _ => unreachable!("type checker missed while statement non bool expression"),
            };
            while repeat {
                match execute_block(body, environment) {
                    ControlFlow::Normal | ControlFlow::Continue => {}
                    ControlFlow::Break => break,
                    ControlFlow::Return(value) => return ControlFlow::Return(value),
                }
                repeat = match evaluate_expression(condition, environment) {
                    Value::Bool(b) => b,
                    _ => unreachable!("type checker missed while statement non bool expression"),
                };
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
                execute_statement(init, environment);
            }
            loop {
                if let Some(condition) = condition {
                    let again = match evaluate_expression(condition, environment) {
                        Value::Bool(b) => b,
                        _ => unreachable!("type checker missed for statement non bool expression"),
                    };
                    if !again {
                        break;
                    }
                }

                match execute_block(body, environment) {
                    ControlFlow::Normal | ControlFlow::Continue => {}
                    ControlFlow::Return(value) => {
                        environment.pop_scope();
                        return ControlFlow::Return(value);
                    }
                    ControlFlow::Break => break,
                }

                if let Some(increment) = increment {
                    execute_statement(increment, environment);
                }
            }
            environment.pop_scope();
        }
        Statement::Scope { body } => {
            return execute_block(body, environment);
        }
        Statement::FunctionDefinition { .. } => {
            unreachable!("function definitions belong in the checked function table")
        }
        Statement::ExpressionStatement { expression } => {
            evaluate_expression(expression, environment);
        }
        Statement::Return { value } => {
            let value = match value {
                Some(expression) => evaluate_expression(expression, environment),
                None => Value::Unit,
            };

            return ControlFlow::Return(value);
        }
        Statement::Break => return ControlFlow::Break,
        Statement::Continue => return ControlFlow::Continue,
        Statement::Pass => (),
    }

    ControlFlow::Normal
}

fn evaluate_expression(expression: &Expression, environment: &mut Environment) -> Value {
    match expression {
        Expression::Literal(val) => val.clone(),
        Expression::Identifier(name) => environment.get(name).clone(),
        Expression::BinaryOp {
            left,
            operator,
            right,
            inferred_type: _,
        } => {
            let left = evaluate_expression(left, environment);
            let right = evaluate_expression(right, environment);
            operations::evaluate_binary_operator(left, operator, right)
        }
        Expression::UnaryOp {
            operator,
            operand,
            inferred_type: _,
        } => operations::evaluate_unary_operator(operand, operator, environment),
        Expression::ResolvedFunctionCall {
            function_id,
            arguments,
        } => {
            let arguments = arguments
                .iter()
                .map(|argument| evaluate_expression(argument, environment))
                .collect();
            let function = environment
                .function_table
                .get(function_id)
                .expect(&format!(
                    "checked program function {:?} is missing from the function table",
                    function_id
                ))
                .clone();
            call_function(&function, arguments, environment)
        }
        Expression::UnresolvedFunctionCall { .. } => {
            unreachable!("unresolved function call reached interpreter")
        }
    }
}

fn call_function(
    function: &Function,
    arguments: Vec<Value>,
    environment: &mut Environment,
) -> Value {
    match &function.implementation {
        FunctionImplementation::Oxide(function_body) => {
            if function.parameters.len() != arguments.len() {
                unreachable!("semantic analyser missed argument count mismatch");
            }

            environment.push_scope();
            for (parameter, argument) in function.parameters.iter().zip(arguments) {
                environment.define(parameter.name.clone(), argument);
            }
            let result = match execute_block(&function_body.statements, environment) {
                ControlFlow::Normal => Value::Unit,
                ControlFlow::Return(value) => value,
                ControlFlow::Break => unreachable!("context checker missed break outside a loop"),
                ControlFlow::Continue => {
                    unreachable!("context checker missed continue outside a loop")
                }
            };
            environment.pop_scope();
            result
        }
        _ => builtins::execute_builtin(function, arguments),
    }
}
