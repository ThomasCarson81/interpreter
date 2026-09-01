use std::time::{self, UNIX_EPOCH};

use syntax::{
    functions::{BuiltinId, Function, FunctionImplementation},
    types::{Number, Type, Value},
};

pub fn execute_builtin(function: &Function, arguments: Vec<Value>) -> Value {
    match &function.implementation {
        FunctionImplementation::Oxide(_) => unreachable!("this function is for builtins only."),
        FunctionImplementation::Builtin(BuiltinId::Print) => {
            for argument in arguments {
                print!("{argument}");
            }
            Value::Unit
        }
        FunctionImplementation::Builtin(BuiltinId::Println) => {
            for argument in arguments {
                println!("{argument}");
            }
            Value::Unit
        }
        FunctionImplementation::Builtin(BuiltinId::ToString) => {
            let [Value::Number(number)] = arguments.as_slice() else {
                unreachable!("semantic analyser missed to_string's numeric argument");
            };
            Value::String(number.to_string())
        }
        FunctionImplementation::Builtin(BuiltinId::Input) => {
            let response = match arguments.as_slice() {
                [] => prompted::input!(),
                [Value::String(message)] => prompted::input!("{}", message),
                _ => unreachable!("semantic analyser missed input's string argument"),
            };
            Value::String(response)
        }
        FunctionImplementation::Builtin(BuiltinId::ToNum(target_type)) => {
            let [value] = arguments.as_slice() else {
                unreachable!("semantic analyser missed to_num argument");
            };

            match value {
                Value::String(s) => Value::Number(parse_number(s, *target_type)),
                Value::Number(n) => Value::Number(convert_number(n, *target_type)),
                _ => unreachable!("semantic analyser allowed invalid argument to to_num"),
            }
        }
        FunctionImplementation::Builtin(BuiltinId::Time) => {
            let current_time = time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("System time went backwards!")
                .as_secs_f64();
            return Value::Number(Number::F64(current_time));
        }
    }
}

pub fn parse_number(string: &str, target: Type) -> Number {
    match target {
        Type::I8 => Number::I8(string.parse().expect("failed to parse number")),
        Type::I16 => Number::I16(string.parse().expect("failed to parse number")),
        Type::I32 => Number::I32(string.parse().expect("failed to parse number")),
        Type::I64 => Number::I64(string.parse().expect("failed to parse number")),

        Type::U8 => Number::U8(string.parse().expect("failed to parse number")),
        Type::U16 => Number::U16(string.parse().expect("failed to parse number")),
        Type::U32 => Number::U32(string.parse().expect("failed to parse number")),
        Type::U64 => Number::U64(string.parse().expect("failed to parse number")),

        Type::F32 => Number::F32(string.parse().expect("failed to parse number")),
        Type::F64 => Number::F64(string.parse().expect("failed to parse number")),

        _ => unreachable!("to_num target must be numeric"),
    }
}

pub fn convert_number(number: &Number, target: Type) -> Number {
    match number {
        Number::I8(n) => parse_number(&n.to_string(), target),
        Number::I16(n) => parse_number(&n.to_string(), target),
        Number::I32(n) => parse_number(&n.to_string(), target),
        Number::I64(n) => parse_number(&n.to_string(), target),
        Number::U8(n) => parse_number(&n.to_string(), target),
        Number::U16(n) => parse_number(&n.to_string(), target),
        Number::U32(n) => parse_number(&n.to_string(), target),
        Number::U64(n) => parse_number(&n.to_string(), target),
        Number::F32(n) => parse_number(&n.to_string(), target),
        Number::F64(n) => parse_number(&n.to_string(), target),
        _ => unreachable!(),
    }
}
