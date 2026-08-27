use crate::{Environment, RuntimeError};
use prompted::input;
use syntax::{Number, Value};

pub fn register_all(environment: &mut Environment) {
    environment.define_builtin_func(String::from("print"), builtin_print);
    environment.define_builtin_func(String::from("println"), builtin_println);
    environment.define_builtin_func(String::from("input"), cmd_input);
    environment.define_builtin_func(String::from("as_i8"), as_i8);
    environment.define_builtin_func(String::from("as_i16"), as_i16);
    environment.define_builtin_func(String::from("as_i32"), as_i32);
    environment.define_builtin_func(String::from("as_i64"), as_i64);
    environment.define_builtin_func(String::from("as_u8"), as_u8);
    environment.define_builtin_func(String::from("as_u16"), as_u16);
    environment.define_builtin_func(String::from("as_u32"), as_u32);
    environment.define_builtin_func(String::from("as_u64"), as_u64);
    environment.define_builtin_func(String::from("as_f32"), as_f32);
    environment.define_builtin_func(String::from("as_f64"), as_f64);
}

fn builtin_print(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    for value in args {
        print!("{value}");
    }
    Ok(Value::Unit)
}

fn builtin_println(
    args: Vec<Value>,
    _environment: &mut Environment,
) -> Result<Value, RuntimeError> {
    for value in args {
        println!("{value}");
    }
    Ok(Value::Unit)
}

// number casts - will be removed when a proper cast system is developed

fn as_i8(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentCountMismatch(
            String::from("as_i8"),
            1,
            args.len(),
        ));
    }
    match args[0] {
        Value::Number(number) => match number {
            syntax::Number::I8(n) => Ok(Value::Number(Number::I8(n))),
            syntax::Number::I16(n) => Ok(Value::Number(Number::I8(n as i8))),
            syntax::Number::I32(n) => Ok(Value::Number(Number::I8(n as i8))),
            syntax::Number::I64(n) => Ok(Value::Number(Number::I8(n as i8))),
            syntax::Number::U8(n) => Ok(Value::Number(Number::I8(n as i8))),
            syntax::Number::U16(n) => Ok(Value::Number(Number::I8(n as i8))),
            syntax::Number::U32(n) => Ok(Value::Number(Number::I8(n as i8))),
            syntax::Number::U64(n) => Ok(Value::Number(Number::I8(n as i8))),
            syntax::Number::F32(n) => Ok(Value::Number(Number::I8(n as i8))),
            syntax::Number::F64(n) => Ok(Value::Number(Number::I8(n as i8))),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => Err(RuntimeError::TypeError(
            String::from("only number types may be cast."),
        )),
    }
}

fn as_i16(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentCountMismatch(
            String::from("as_i16"),
            1,
            args.len(),
        ));
    }
    match args[0] {
        Value::Number(number) => match number {
            syntax::Number::I8(n) => Ok(Value::Number(Number::I16(n as i16))),
            syntax::Number::I16(n) => Ok(Value::Number(Number::I16(n))),
            syntax::Number::I32(n) => Ok(Value::Number(Number::I16(n as i16))),
            syntax::Number::I64(n) => Ok(Value::Number(Number::I16(n as i16))),
            syntax::Number::U8(n) => Ok(Value::Number(Number::I16(n as i16))),
            syntax::Number::U16(n) => Ok(Value::Number(Number::I16(n as i16))),
            syntax::Number::U32(n) => Ok(Value::Number(Number::I16(n as i16))),
            syntax::Number::U64(n) => Ok(Value::Number(Number::I16(n as i16))),
            syntax::Number::F32(n) => Ok(Value::Number(Number::I16(n as i16))),
            syntax::Number::F64(n) => Ok(Value::Number(Number::I16(n as i16))),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => Err(RuntimeError::TypeError(
            String::from("only number types may be cast."),
        )),
    }
}

fn as_i32(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentCountMismatch(
            String::from("as_i32"),
            1,
            args.len(),
        ));
    }
    match args[0] {
        Value::Number(number) => match number {
            syntax::Number::I8(n) => Ok(Value::Number(Number::I32(n as i32))),
            syntax::Number::I16(n) => Ok(Value::Number(Number::I32(n as i32))),
            syntax::Number::I32(n) => Ok(Value::Number(Number::I32(n))),
            syntax::Number::I64(n) => Ok(Value::Number(Number::I32(n as i32))),
            syntax::Number::U8(n) => Ok(Value::Number(Number::I32(n as i32))),
            syntax::Number::U16(n) => Ok(Value::Number(Number::I32(n as i32))),
            syntax::Number::U32(n) => Ok(Value::Number(Number::I32(n as i32))),
            syntax::Number::U64(n) => Ok(Value::Number(Number::I32(n as i32))),
            syntax::Number::F32(n) => Ok(Value::Number(Number::I32(n as i32))),
            syntax::Number::F64(n) => Ok(Value::Number(Number::I32(n as i32))),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => Err(RuntimeError::TypeError(
            String::from("only number types may be cast."),
        )),
    }
}

fn as_i64(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentCountMismatch(
            String::from("as_i64"),
            1,
            args.len(),
        ));
    }
    match args[0] {
        Value::Number(number) => match number {
            syntax::Number::I8(n) => Ok(Value::Number(Number::I64(n as i64))),
            syntax::Number::I16(n) => Ok(Value::Number(Number::I64(n as i64))),
            syntax::Number::I32(n) => Ok(Value::Number(Number::I64(n as i64))),
            syntax::Number::I64(n) => Ok(Value::Number(Number::I64(n as i64))),
            syntax::Number::U8(n) => Ok(Value::Number(Number::I64(n as i64))),
            syntax::Number::U16(n) => Ok(Value::Number(Number::I64(n as i64))),
            syntax::Number::U32(n) => Ok(Value::Number(Number::I64(n as i64))),
            syntax::Number::U64(n) => Ok(Value::Number(Number::I64(n as i64))),
            syntax::Number::F32(n) => Ok(Value::Number(Number::I64(n as i64))),
            syntax::Number::F64(n) => Ok(Value::Number(Number::I64(n as i64))),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => Err(RuntimeError::TypeError(
            String::from("only number types may be cast."),
        )),
    }
}

fn as_u8(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentCountMismatch(
            String::from("as_u8"),
            1,
            args.len(),
        ));
    }
    match args[0] {
        Value::Number(number) => match number {
            syntax::Number::I8(n) => Ok(Value::Number(Number::U8(n as u8))),
            syntax::Number::I16(n) => Ok(Value::Number(Number::U8(n as u8))),
            syntax::Number::I32(n) => Ok(Value::Number(Number::U8(n as u8))),
            syntax::Number::I64(n) => Ok(Value::Number(Number::U8(n as u8))),
            syntax::Number::U8(n) => Ok(Value::Number(Number::U8(n))),
            syntax::Number::U16(n) => Ok(Value::Number(Number::U8(n as u8))),
            syntax::Number::U32(n) => Ok(Value::Number(Number::U8(n as u8))),
            syntax::Number::U64(n) => Ok(Value::Number(Number::U8(n as u8))),
            syntax::Number::F32(n) => Ok(Value::Number(Number::U8(n as u8))),
            syntax::Number::F64(n) => Ok(Value::Number(Number::U8(n as u8))),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => Err(RuntimeError::TypeError(
            String::from("only number types may be cast."),
        )),
    }
}

fn as_u16(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentCountMismatch(
            String::from("as_u16"),
            1,
            args.len(),
        ));
    }
    match args[0] {
        Value::Number(number) => match number {
            syntax::Number::I8(n) => Ok(Value::Number(Number::U16(n as u16))),
            syntax::Number::I16(n) => Ok(Value::Number(Number::U16(n as u16))),
            syntax::Number::I32(n) => Ok(Value::Number(Number::U16(n as u16))),
            syntax::Number::I64(n) => Ok(Value::Number(Number::U16(n as u16))),
            syntax::Number::U8(n) => Ok(Value::Number(Number::U16(n as u16))),
            syntax::Number::U16(n) => Ok(Value::Number(Number::U16(n))),
            syntax::Number::U32(n) => Ok(Value::Number(Number::U16(n as u16))),
            syntax::Number::U64(n) => Ok(Value::Number(Number::U16(n as u16))),
            syntax::Number::F32(n) => Ok(Value::Number(Number::U16(n as u16))),
            syntax::Number::F64(n) => Ok(Value::Number(Number::U16(n as u16))),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => Err(RuntimeError::TypeError(
            String::from("only number types may be cast."),
        )),
    }
}

fn as_u32(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentCountMismatch(
            String::from("as_u32"),
            1,
            args.len(),
        ));
    }
    match args[0] {
        Value::Number(number) => match number {
            syntax::Number::I8(n) => Ok(Value::Number(Number::U32(n as u32))),
            syntax::Number::I16(n) => Ok(Value::Number(Number::U32(n as u32))),
            syntax::Number::I32(n) => Ok(Value::Number(Number::U32(n as u32))),
            syntax::Number::I64(n) => Ok(Value::Number(Number::U32(n as u32))),
            syntax::Number::U8(n) => Ok(Value::Number(Number::U32(n as u32))),
            syntax::Number::U16(n) => Ok(Value::Number(Number::U32(n as u32))),
            syntax::Number::U32(n) => Ok(Value::Number(Number::U32(n))),
            syntax::Number::U64(n) => Ok(Value::Number(Number::U32(n as u32))),
            syntax::Number::F32(n) => Ok(Value::Number(Number::U32(n as u32))),
            syntax::Number::F64(n) => Ok(Value::Number(Number::U32(n as u32))),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => Err(RuntimeError::TypeError(
            String::from("only number types may be cast."),
        )),
    }
}

fn as_u64(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentCountMismatch(
            String::from("as_u64"),
            1,
            args.len(),
        ));
    }
    match args[0] {
        Value::Number(number) => match number {
            syntax::Number::I8(n) => Ok(Value::Number(Number::U64(n as u64))),
            syntax::Number::I16(n) => Ok(Value::Number(Number::U64(n as u64))),
            syntax::Number::I32(n) => Ok(Value::Number(Number::U64(n as u64))),
            syntax::Number::I64(n) => Ok(Value::Number(Number::U64(n as u64))),
            syntax::Number::U8(n) => Ok(Value::Number(Number::U64(n as u64))),
            syntax::Number::U16(n) => Ok(Value::Number(Number::U64(n as u64))),
            syntax::Number::U32(n) => Ok(Value::Number(Number::U64(n as u64))),
            syntax::Number::U64(n) => Ok(Value::Number(Number::U64(n))),
            syntax::Number::F32(n) => Ok(Value::Number(Number::U64(n as u64))),
            syntax::Number::F64(n) => Ok(Value::Number(Number::U64(n as u64))),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => Err(RuntimeError::TypeError(
            String::from("only number types may be cast."),
        )),
    }
}

fn as_f32(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentCountMismatch(
            String::from("as_f32"),
            1,
            args.len(),
        ));
    }
    match args[0] {
        Value::Number(number) => match number {
            syntax::Number::I8(n) => Ok(Value::Number(Number::F32(n as f32))),
            syntax::Number::I16(n) => Ok(Value::Number(Number::F32(n as f32))),
            syntax::Number::I32(n) => Ok(Value::Number(Number::F32(n as f32))),
            syntax::Number::I64(n) => Ok(Value::Number(Number::F32(n as f32))),
            syntax::Number::U8(n) => Ok(Value::Number(Number::F32(n as f32))),
            syntax::Number::U16(n) => Ok(Value::Number(Number::F32(n as f32))),
            syntax::Number::U32(n) => Ok(Value::Number(Number::F32(n as f32))),
            syntax::Number::U64(n) => Ok(Value::Number(Number::F32(n as f32))),
            syntax::Number::F32(n) => Ok(Value::Number(Number::F32(n))),
            syntax::Number::F64(n) => Ok(Value::Number(Number::F32(n as f32))),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => Err(RuntimeError::TypeError(
            String::from("only number types may be cast."),
        )),
    }
}

fn as_f64(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentCountMismatch(
            String::from("as_f64"),
            1,
            args.len(),
        ));
    }
    match args[0] {
        Value::Number(number) => match number {
            syntax::Number::I8(n) => Ok(Value::Number(Number::F64(n as f64))),
            syntax::Number::I16(n) => Ok(Value::Number(Number::F64(n as f64))),
            syntax::Number::I32(n) => Ok(Value::Number(Number::F64(n as f64))),
            syntax::Number::I64(n) => Ok(Value::Number(Number::F64(n as f64))),
            syntax::Number::U8(n) => Ok(Value::Number(Number::F64(n as f64))),
            syntax::Number::U16(n) => Ok(Value::Number(Number::F64(n as f64))),
            syntax::Number::U32(n) => Ok(Value::Number(Number::F64(n as f64))),
            syntax::Number::U64(n) => Ok(Value::Number(Number::F64(n as f64))),
            syntax::Number::F32(n) => Ok(Value::Number(Number::F64(n as f64))),
            syntax::Number::F64(n) => Ok(Value::Number(Number::F64(n))),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => Err(RuntimeError::TypeError(
            String::from("only number types may be cast."),
        )),
    }
}

fn cmd_input(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() > 1 {
        return Err(RuntimeError::ArgumentCountMismatch(
            String::from("input"),
            1,
            args.len(),
        ));
    }
    let user_input = input!("{}", args[0]);

    Ok(Value::String(user_input))
}
