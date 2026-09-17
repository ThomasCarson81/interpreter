use std::time::{self, UNIX_EPOCH};

use syntax::{
    functions::{Function, FunctionImplementation},
    stdlib::BuiltinId,
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
                Value::String(s) => Value::Number(parse_number(s, target_type.clone())),
                Value::Number(n) => Value::Number(convert_number(n, target_type.clone())),
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
        FunctionImplementation::Builtin(BuiltinId::Abs) => {
            let [value] = arguments.as_slice() else {
                unreachable!("semantic analyser allowed incorrect parameters to abs()");
            };
            match value {
                Value::Number(number) => match number {
                    Number::I8(n) => Value::Number(Number::I8(i8::abs(*n))),
                    Number::I16(n) => Value::Number(Number::I16(i16::abs(*n))),
                    Number::I32(n) => Value::Number(Number::I32(i32::abs(*n))),
                    Number::I64(n) => Value::Number(Number::I64(i64::abs(*n))),
                    Number::F32(n) => Value::Number(Number::F32(f32::abs(*n))),
                    Number::F64(n) => Value::Number(Number::F64(f64::abs(*n))),
                    Number::U8(_) | Number::U16(_) | Number::U32(_) | Number::U64(_) => {
                        unreachable!("semantic analyser allowed unsigned types to abs()");
                    }
                    Number::IntegerLiteral(_) | Number::FloatLiteral(_) => todo!(),
                },
                Value::String(_) | Value::Bool(_) | Value::Unit => {
                    unreachable!("semantic analyser allowed non number types to abs()")
                }
            }
        }
        FunctionImplementation::Builtin(BuiltinId::Min) => {
            let [a, b] = arguments.as_slice() else {
                unreachable!("semantic analyser allowed incorrect parameters to min()");
            };
            match (a, b) {
                (Value::Number(a), Value::Number(b)) => match (a, b) {
                    (Number::I8(a), Number::I8(b)) => Value::Number(Number::I8(i8::min(*a, *b))),
                    (Number::I16(a), Number::I16(b)) => {
                        Value::Number(Number::I16(i16::min(*a, *b)))
                    }
                    (Number::I32(a), Number::I32(b)) => {
                        Value::Number(Number::I32(i32::min(*a, *b)))
                    }
                    (Number::I64(a), Number::I64(b)) => {
                        Value::Number(Number::I64(i64::min(*a, *b)))
                    }
                    (Number::U8(a), Number::U8(b)) => Value::Number(Number::U8(u8::min(*a, *b))),
                    (Number::U16(a), Number::U16(b)) => {
                        Value::Number(Number::U16(u16::min(*a, *b)))
                    }
                    (Number::U32(a), Number::U32(b)) => {
                        Value::Number(Number::U32(u32::min(*a, *b)))
                    }
                    (Number::U64(a), Number::U64(b)) => {
                        Value::Number(Number::U64(u64::min(*a, *b)))
                    }
                    (Number::F32(a), Number::F32(b)) => {
                        Value::Number(Number::F32(f32::min(*a, *b)))
                    }
                    (Number::F64(a), Number::F64(b)) => {
                        Value::Number(Number::F64(f64::min(*a, *b)))
                    }
                    _ => unreachable!("semantic analyser allowed different numeric types to min()"),
                },
                _ => unreachable!("semantic analyser allowed non number types to min()"),
            }
        }
        FunctionImplementation::Builtin(BuiltinId::Max) => {
            let [a, b] = arguments.as_slice() else {
                unreachable!("semantic analyser allowed incorrect parameters to max()");
            };
            match (a, b) {
                (Value::Number(a), Value::Number(b)) => match (a, b) {
                    (Number::I8(a), Number::I8(b)) => Value::Number(Number::I8(i8::max(*a, *b))),
                    (Number::I16(a), Number::I16(b)) => {
                        Value::Number(Number::I16(i16::max(*a, *b)))
                    }
                    (Number::I32(a), Number::I32(b)) => {
                        Value::Number(Number::I32(i32::max(*a, *b)))
                    }
                    (Number::I64(a), Number::I64(b)) => {
                        Value::Number(Number::I64(i64::max(*a, *b)))
                    }
                    (Number::U8(a), Number::U8(b)) => Value::Number(Number::U8(u8::max(*a, *b))),
                    (Number::U16(a), Number::U16(b)) => {
                        Value::Number(Number::U16(u16::max(*a, *b)))
                    }
                    (Number::U32(a), Number::U32(b)) => {
                        Value::Number(Number::U32(u32::max(*a, *b)))
                    }
                    (Number::U64(a), Number::U64(b)) => {
                        Value::Number(Number::U64(u64::max(*a, *b)))
                    }
                    (Number::F32(a), Number::F32(b)) => {
                        Value::Number(Number::F32(f32::max(*a, *b)))
                    }
                    (Number::F64(a), Number::F64(b)) => {
                        Value::Number(Number::F64(f64::max(*a, *b)))
                    }
                    _ => unreachable!("semantic analyser allowed different numeric types to max()"),
                },
                _ => unreachable!("semantic analyser allowed non number types to max()"),
            }
        }
        FunctionImplementation::Builtin(BuiltinId::Clamp) => {
            let [value, min, max] = arguments.as_slice() else {
                unreachable!("semantic analyser allowed incorrect parameters to clamp()");
            };

            match (value, min, max) {
                (Value::Number(value), Value::Number(min), Value::Number(max)) => {
                    let clamped = match (value, min, max) {
                        (Number::I8(value), Number::I8(min), Number::I8(max)) => {
                            Value::Number(Number::I8(i8::clamp(*value, *min, *max)))
                        }
                        (Number::I16(value), Number::I16(min), Number::I16(max)) => {
                            Value::Number(Number::I16(i16::clamp(*value, *min, *max)))
                        }
                        (Number::I32(value), Number::I32(min), Number::I32(max)) => {
                            Value::Number(Number::I32(i32::clamp(*value, *min, *max)))
                        }
                        (Number::I64(value), Number::I64(min), Number::I64(max)) => {
                            Value::Number(Number::I64(i64::clamp(*value, *min, *max)))
                        }
                        (Number::U8(value), Number::U8(min), Number::U8(max)) => {
                            Value::Number(Number::U8(u8::clamp(*value, *min, *max)))
                        }
                        (Number::U16(value), Number::U16(min), Number::U16(max)) => {
                            Value::Number(Number::U16(u16::clamp(*value, *min, *max)))
                        }
                        (Number::U32(value), Number::U32(min), Number::U32(max)) => {
                            Value::Number(Number::U32(u32::clamp(*value, *min, *max)))
                        }
                        (Number::U64(value), Number::U64(min), Number::U64(max)) => {
                            Value::Number(Number::U64(u64::clamp(*value, *min, *max)))
                        }
                        (Number::F32(value), Number::F32(min), Number::F32(max)) => {
                            Value::Number(Number::F32(f32::clamp(*value, *min, *max)))
                        }
                        (Number::F64(value), Number::F64(min), Number::F64(max)) => {
                            Value::Number(Number::F64(f64::clamp(*value, *min, *max)))
                        }
                        _ => unreachable!(
                            "semantic analyser allowed different numeric types to clamp()"
                        ),
                    };
                    clamped
                }
                _ => unreachable!("semantic analyser allowed non number types to clamp()"),
            }
        }
        FunctionImplementation::Builtin(BuiltinId::Sqrt) => {
            let [Value::Number(number)] = arguments.as_slice() else {
                unreachable!("semantic analyser missed sqrt's numeric argument");
            };
            match number {
                Number::F32(n) => Value::Number(Number::F32(f32::sqrt(*n))),
                Number::F64(n) => Value::Number(Number::F64(f64::sqrt(*n))),
                _ => unreachable!("semantic analyser allowed invalid argument to sqrt()"),
            }
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
