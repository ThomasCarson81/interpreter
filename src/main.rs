use std::{collections::HashMap, env, time};
use syntax::{
    ast::{Expression, Program, Statement, TokenType},
    functions::{Function, FunctionImplementation, FunctionTable},
    stdlib::builtin_functions,
    types::{Number, Value},
};
mod builtins;
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

type BuiltInFunction = fn(Vec<Value>, &mut Environment) -> Value;

// #[derive(Clone)]
// enum Function {
//     UserDefined {
//         parameters: Vec<FunctionParameter>,
//         body: Vec<Statement>,
//     },
//     BuiltIn {
//         function: BuiltInFunction,
//     },
// }
// impl Function {
//     fn call(&self, arguments: Vec<Value>, environment: &mut Environment) -> Value {
//         match self {
//             Function::UserDefined { parameters, body } => {
//                 let num_params = parameters.len();
//                 let num_args = arguments.len();
//                 if num_params != num_args {
//                     unreachable!("semantic analyser missed argument count mismatch");
//                 }
//                 environment.push_scope();
//                 for (parameter, argument) in parameters.iter().zip(arguments) {
//                     environment.define(parameter.name.clone(), argument);
//                 }
//                 match execute_block(&body, environment) {
//                     ControlFlow::Normal => {}
//                     ControlFlow::Return(value) => {
//                         environment.pop_scope();
//                         return value;
//                     }
//                     ControlFlow::Break => {
//                         environment.pop_scope();
//                         unreachable!("illegal control flow: break");
//                     }
//                     ControlFlow::Continue => {
//                         environment.pop_scope();
//                         unreachable!("illegal control flow: continue");
//                     }
//                 }
//                 environment.pop_scope();
//                 Value::Unit
//             }
//             Function::BuiltIn { function, .. } => function(arguments, environment),
//         }
//     }
// }

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

    // get functiontable from file
    let function_table = FunctionTable {
        functions: builtin_functions(),
    };

    let mut environment = Environment { scopes: Vec::new() };
    // builtins::register_all(&mut environment);
    environment.push_scope();
    let start_time = time::Instant::now();
    execute_program(&program, &mut environment);
    let end_time = time::Instant::now();
    let total_time = end_time - start_time;

    println!("\nProgram finished executing in {:?}", total_time);

    println!("Global variables: ");
    for scope in environment.scopes {
        for kv in scope {
            println!("{}={}", kv.0, kv.1);
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

fn execute_program(program: &Program, environment: &mut Environment) {
    for statement in &program.statements {
        execute_statement(statement, environment);
    }
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
        Statement::FunctionDefinition {
            identifier: _,
            parameters,
            body,
            return_type: _,
        } => {
            // let _func = Function::UserDefined {
            //     parameters: parameters.clone(),
            //     body: body.clone(),
            // };
            // environment.define_func(identifier.clone(), func);
            todo!("run main() instead of interpreting full ast");
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
        } => {
            let left = evaluate_expression(left, environment);
            let right = evaluate_expression(right, environment);
            evaluate_binary_operator(left, operator, right)
        }
        Expression::UnaryOp { operator, operand } => {
            evaluate_unary_operator(operand, operator, environment)
        }
        Expression::ResolvedFunctionCall {
            function_id,
            arguments,
        } => {
            // let arguments = arguments
            //     .iter()
            //     .map(|expr| evaluate_expression(expr, environment))
            //     .collect();
            // let func = environment
            //     .function_table
            //     .get(function_id.clone())
            //     .expect("function table missing function")
            //     .clone();
            // call_function(func, arguments, environment)
            Value::Unit
        }
        Expression::UnresolvedFunctionCall {
            identifier,
            arguments,
        } => unreachable!("unresolved function call reached interpreter"),
    }
}

fn evaluate_binary_operator(left: Value, operator: &TokenType, right: Value) -> Value {
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
        _ => unreachable!(
            "semantic analyser missed unknown binary operator: {}",
            operator
        ),
    }
}

fn evaluate_unary_operator(
    operand: &Expression,
    operator: &TokenType,
    environment: &mut Environment,
) -> Value {
    match operator {
        TokenType::Increment => evaluate_increment(operand, environment),
        TokenType::Decrement => evaluate_decrement(operand, environment),
        TokenType::Not => evaluate_not_operator(&evaluate_expression(operand, environment)),
        TokenType::Minus => evaluate_unary_minus(&evaluate_expression(operand, environment)),
        _ => unreachable!(
            "semantic analyser missed unknown unary operator: {}",
            operator
        ),
    }
}

fn evaluate_plus(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Number(add_numbers(*left, *right)),
        (Value::String(left), Value::String(right)) => Value::String(left.to_owned() + right),
        _ => unreachable!(
            "type checker did not catch incorrect + operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_minus(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => {
            Value::Number(subtract_numbers(*left, *right))
        }
        _ => unreachable!(
            "type checker did not catch incorrect - operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_multiply(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => {
            Value::Number(multiply_numbers(*left, *right))
        }
        _ => unreachable!(
            "type checker did not catch incorrect * operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_divide(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => {
            Value::Number(divide_numbers(*left, *right)) // will panic on x / 0
        }
        _ => unreachable!(
            "type checker did not catch incorrect / operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_greater_than(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Bool(left > right),
        _ => unreachable!(
            "type checker did not catch incorrect > operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_greater_than_or_equal_to(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Bool(left >= right),
        _ => unreachable!(
            "type checker did not catch incorrect >= operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_less_than(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Bool(left < right),
        _ => unreachable!(
            "type checker did not catch incorrect < operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_less_than_or_equal_to(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Bool(left <= right),
        _ => unreachable!(
            "type checker did not catch incorrect <= operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_equal_to(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Bool(left == right),
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left == right),
        (Value::String(left), Value::String(right)) => Value::Bool(left == right),
        _ => unreachable!(
            "type checker did not catch incorrect == operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_not_equal_to(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Bool(left != right),
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left != right),
        (Value::String(left), Value::String(right)) => Value::Bool(left == right),
        _ => unreachable!(
            "type checker did not catch incorrect != operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_logical_and(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(*left && *right),
        _ => unreachable!(
            "type checker did not catch incorrect && operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_logical_or(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(*left || *right),
        _ => unreachable!(
            "type checker did not catch incorrect || operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn evaluate_increment(operand: &Expression, environment: &mut Environment) -> Value {
    match operand {
        Expression::Identifier(name) => {
            let num = environment.get(name);
            let Value::Number(n) = num else {
                unreachable!("type checker did not catch incorrect ++ operand: {:?}", num)
            };
            let new_num = Value::Number(increment_number(*n));
            environment.assign(name, new_num.clone());
            new_num
        }
        _ => unreachable!("type checker did not catch non-identifier ++ operand"),
    }
}

fn evaluate_decrement(operand: &Expression, environment: &mut Environment) -> Value {
    match operand {
        Expression::Identifier(name) => {
            let num = environment.get(name);
            let Value::Number(n) = num else {
                unreachable!("type checker did not catch incorrect -- operand: {:?}", num)
            };
            let new_num = Value::Number(decrement_number(*n));
            environment.assign(name, new_num.clone());
            new_num
        }
        _ => unreachable!("type checker did not catch non-identifier -- operand"),
    }
}

fn evaluate_not_operator(operand: &Value) -> Value {
    match operand {
        Value::Bool(b) => Value::Bool(!b),
        _ => unreachable!(
            "type checker did not catch incorrect ! operand: {:?}",
            operand
        ),
    }
}

fn evaluate_unary_minus(operand: &Value) -> Value {
    match operand {
        Value::Number(num) => Value::Number(negate_number(*num)),
        _ => unreachable!(
            "type checker did not catch incorrect unary - operand: {:?}",
            operand
        ),
    }
}

fn add_numbers(left: Number, right: Number) -> Number {
    match (&left, &right) {
        (Number::I8(left), Number::I8(right)) => Number::I8(left + right),
        (Number::I16(left), Number::I16(right)) => Number::I16(left + right),
        (Number::I32(left), Number::I32(right)) => Number::I32(left + right),
        (Number::I64(left), Number::I64(right)) => Number::I64(left + right),

        (Number::U8(left), Number::U8(right)) => Number::U8(left + right),
        (Number::U16(left), Number::U16(right)) => Number::U16(left + right),
        (Number::U32(left), Number::U32(right)) => Number::U32(left + right),
        (Number::U64(left), Number::U64(right)) => Number::U64(left + right),

        (Number::F32(left), Number::F32(right)) => Number::F32(left + right),
        (Number::F64(left), Number::F64(right)) => Number::F64(left + right),
        _ => unreachable!(
            "type checker did not catch invalid + operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn subtract_numbers(left: Number, right: Number) -> Number {
    match (&left, &right) {
        (Number::I8(left), Number::I8(right)) => Number::I8(left - right),
        (Number::I16(left), Number::I16(right)) => Number::I16(left - right),
        (Number::I32(left), Number::I32(right)) => Number::I32(left - right),
        (Number::I64(left), Number::I64(right)) => Number::I64(left - right),

        (Number::U8(left), Number::U8(right)) => Number::U8(left - right),
        (Number::U16(left), Number::U16(right)) => Number::U16(left - right),
        (Number::U32(left), Number::U32(right)) => Number::U32(left - right),
        (Number::U64(left), Number::U64(right)) => Number::U64(left - right),

        (Number::F32(left), Number::F32(right)) => Number::F32(left - right),
        (Number::F64(left), Number::F64(right)) => Number::F64(left - right),
        _ => unreachable!(
            "type checker did not catch incorrect - operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn multiply_numbers(left: Number, right: Number) -> Number {
    match (&left, &right) {
        (Number::I8(left), Number::I8(right)) => Number::I8(left * right),
        (Number::I16(left), Number::I16(right)) => Number::I16(left * right),
        (Number::I32(left), Number::I32(right)) => Number::I32(left * right),
        (Number::I64(left), Number::I64(right)) => Number::I64(left * right),

        (Number::U8(left), Number::U8(right)) => Number::U8(left * right),
        (Number::U16(left), Number::U16(right)) => Number::U16(left * right),
        (Number::U32(left), Number::U32(right)) => Number::U32(left * right),
        (Number::U64(left), Number::U64(right)) => Number::U64(left * right),

        (Number::F32(left), Number::F32(right)) => Number::F32(left * right),
        (Number::F64(left), Number::F64(right)) => Number::F64(left * right),
        _ => unreachable!(
            "type checker did not catch incorrect * operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn divide_numbers(left: Number, right: Number) -> Number {
    match (&left, &right) {
        (Number::I8(left), Number::I8(right)) => Number::I8(left / right),
        (Number::I16(left), Number::I16(right)) => Number::I16(left / right),
        (Number::I32(left), Number::I32(right)) => Number::I32(left / right),
        (Number::I64(left), Number::I64(right)) => Number::I64(left / right),

        (Number::U8(left), Number::U8(right)) => Number::U8(left / right),
        (Number::U16(left), Number::U16(right)) => Number::U16(left / right),
        (Number::U32(left), Number::U32(right)) => Number::U32(left / right),
        (Number::U64(left), Number::U64(right)) => Number::U64(left / right),

        (Number::F32(left), Number::F32(right)) => Number::F32(left / right),
        (Number::F64(left), Number::F64(right)) => Number::F64(left / right),
        _ => unreachable!(
            "type checker did not catch incorrect + operands: {:?} & {:?}",
            left, right
        ),
    }
}

fn negate_number(number: Number) -> Number {
    match number {
        Number::I8(n) => Number::I8(-n),
        Number::I16(n) => Number::I16(-n),
        Number::I32(n) => Number::I32(-n),
        Number::I64(n) => Number::I64(-n),
        Number::F32(n) => Number::F32(-n),
        Number::F64(n) => Number::F64(-n),
        _ => unreachable!(
            "type checker did not catch incorrect unary - operand: {:?}",
            number
        ),
    }
}

fn increment_number(number: Number) -> Number {
    match number {
        Number::I8(n) => Number::I8(n + 1),
        Number::I16(n) => Number::I16(n + 1),
        Number::I32(n) => Number::I32(n + 1),
        Number::I64(n) => Number::I64(n + 1),

        Number::U8(n) => Number::U8(n + 1),
        Number::U16(n) => Number::U16(n + 1),
        Number::U32(n) => Number::U32(n + 1),
        Number::U64(n) => Number::U64(n + 1),

        Number::F32(n) => Number::F32(n + 1.0),
        Number::F64(n) => Number::F64(n + 1.0),

        _ => unreachable!(
            "type checker did not catch incorrect ++ operand: {:?}",
            number
        ),
    }
}

fn decrement_number(number: Number) -> Number {
    match number {
        Number::I8(n) => Number::I8(n - 1),
        Number::I16(n) => Number::I16(n - 1),
        Number::I32(n) => Number::I32(n - 1),
        Number::I64(n) => Number::I64(n - 1),

        Number::U8(n) => Number::U8(n - 1),
        Number::U16(n) => Number::U16(n - 1),
        Number::U32(n) => Number::U32(n - 1),
        Number::U64(n) => Number::U64(n - 1),

        Number::F32(n) => Number::F32(n - 1.0),
        Number::F64(n) => Number::F64(n - 1.0),

        _ => unreachable!(
            "type checker did not catch incorrect -- operand: {:?}",
            number
        ),
    }
}

fn call_function(
    function: &Function,
    arguments: Vec<Value>,
    environment: &mut Environment,
) -> Value {
    match &function.implementation {
        FunctionImplementation::Oxide(function_body) => {}
        FunctionImplementation::Builtin(builtin_id) => {}
    }

    Value::Unit
}
