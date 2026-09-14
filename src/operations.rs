use crate::Environment;
use syntax::{
    ast::{Expression, TokenType},
    types::{Number, Value},
};

use crate::evaluate_expression;

pub fn evaluate_binary_operator(
    left: &Expression,
    operator: &TokenType,
    right: &Expression,
    environment: &mut Environment,
) -> Value {
    let left_value = evaluate_expression(left, environment);
    let right_value = evaluate_expression(right, environment);
    match operator {
        TokenType::Plus => evaluate_plus(left_value, right_value),
        TokenType::Minus => evaluate_minus(left_value, right_value),
        TokenType::Asterisk => evaluate_multiply(left_value, right_value),
        TokenType::Slash => evaluate_divide(left_value, right_value),
        TokenType::Modulo => evaluate_modulo(left_value, right_value),
        TokenType::GreaterThan => evaluate_greater_than(left_value, right_value),
        TokenType::LessThan => evaluate_less_than(left_value, right_value),
        TokenType::GreaterThanOrEqualTo => {
            evaluate_greater_than_or_equal_to(left_value, right_value)
        }
        TokenType::LessThanOrEqualTo => evaluate_less_than_or_equal_to(left_value, right_value),
        TokenType::EqualTo => evaluate_equal_to(left_value, right_value),
        TokenType::NotEqualTo => evaluate_not_equal_to(left_value, right_value),
        TokenType::LogicalOr => evaluate_logical_or(left_value, right_value),
        TokenType::LogicalAnd => evaluate_logical_and(left_value, right_value),
        TokenType::Walrus => evaluate_walrus(left, right_value, environment),
        _ => unreachable!(
            "semantic analyser missed unknown binary operator: {}",
            operator
        ),
    }
}

pub fn evaluate_unary_operator(
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

pub fn evaluate_plus(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Number(add_numbers(*left, *right)),
        (Value::String(left), Value::String(right)) => Value::String(left.to_owned() + right),
        _ => unreachable!(
            "type checker did not catch incorrect + operands: {:?} & {:?}",
            left, right
        ),
    }
}

pub fn evaluate_minus(left: Value, right: Value) -> Value {
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

pub fn evaluate_multiply(left: Value, right: Value) -> Value {
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

pub fn evaluate_divide(left: Value, right: Value) -> Value {
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

pub fn evaluate_modulo(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => {
            Value::Number(modulo_numbers(*left, *right)) // will panic on x % 0
        }
        _ => unreachable!(
            "type checker did not catch incorrect % operands: {:?} & {:?}",
            left, right
        ),
    }
}

pub fn evaluate_greater_than(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Bool(left > right),
        _ => unreachable!(
            "type checker did not catch incorrect > operands: {:?} & {:?}",
            left, right
        ),
    }
}

pub fn evaluate_greater_than_or_equal_to(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Bool(left >= right),
        _ => unreachable!(
            "type checker did not catch incorrect >= operands: {:?} & {:?}",
            left, right
        ),
    }
}

pub fn evaluate_less_than(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Bool(left < right),
        _ => unreachable!(
            "type checker did not catch incorrect < operands: {:?} & {:?}",
            left, right
        ),
    }
}

pub fn evaluate_less_than_or_equal_to(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Number(left), Value::Number(right)) => Value::Bool(left <= right),
        _ => unreachable!(
            "type checker did not catch incorrect <= operands: {:?} & {:?}",
            left, right
        ),
    }
}

pub fn evaluate_equal_to(left: Value, right: Value) -> Value {
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

pub fn evaluate_not_equal_to(left: Value, right: Value) -> Value {
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

pub fn evaluate_logical_and(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(*left && *right),
        _ => unreachable!(
            "type checker did not catch incorrect && operands: {:?} & {:?}",
            left, right
        ),
    }
}

pub fn evaluate_logical_or(left: Value, right: Value) -> Value {
    match (&left, &right) {
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(*left || *right),
        _ => unreachable!(
            "type checker did not catch incorrect || operands: {:?} & {:?}",
            left, right
        ),
    }
}

pub fn evaluate_walrus(left: &Expression, right: Value, environment: &mut Environment) -> Value {
    match left {
        Expression::Identifier(name) => {
            environment.assign(name, right.clone());
            right
        }
        _ => unreachable!("type checker did not catch non-identifier walrus operand"),
    }
}

pub fn evaluate_increment(operand: &Expression, environment: &mut Environment) -> Value {
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

pub fn evaluate_decrement(operand: &Expression, environment: &mut Environment) -> Value {
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

pub fn evaluate_not_operator(operand: &Value) -> Value {
    match operand {
        Value::Bool(b) => Value::Bool(!b),
        _ => unreachable!(
            "type checker did not catch incorrect ! operand: {:?}",
            operand
        ),
    }
}

pub fn evaluate_unary_minus(operand: &Value) -> Value {
    match operand {
        Value::Number(num) => Value::Number(negate_number(*num)),
        _ => unreachable!(
            "type checker did not catch incorrect unary - operand: {:?}",
            operand
        ),
    }
}

pub fn add_numbers(left: Number, right: Number) -> Number {
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

pub fn subtract_numbers(left: Number, right: Number) -> Number {
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

pub fn multiply_numbers(left: Number, right: Number) -> Number {
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

pub fn divide_numbers(left: Number, right: Number) -> Number {
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

pub fn modulo_numbers(left: Number, right: Number) -> Number {
    match (&left, &right) {
        (Number::I8(left), Number::I8(right)) => Number::I8(left % right),
        (Number::I16(left), Number::I16(right)) => Number::I16(left % right),
        (Number::I32(left), Number::I32(right)) => Number::I32(left % right),
        (Number::I64(left), Number::I64(right)) => Number::I64(left % right),

        (Number::U8(left), Number::U8(right)) => Number::U8(left % right),
        (Number::U16(left), Number::U16(right)) => Number::U16(left % right),
        (Number::U32(left), Number::U32(right)) => Number::U32(left % right),
        (Number::U64(left), Number::U64(right)) => Number::U64(left % right),

        (Number::F32(left), Number::F32(right)) => Number::F32(left % right),
        (Number::F64(left), Number::F64(right)) => Number::F64(left % right),

        _ => unreachable!(
            "type checker did not catch incorrect % operands: {:?} & {:?}",
            left, right
        ),
    }
}

pub fn negate_number(number: Number) -> Number {
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

pub fn increment_number(number: Number) -> Number {
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

pub fn decrement_number(number: Number) -> Number {
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
