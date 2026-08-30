use crate::Environment;
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

fn builtin_print(args: Vec<Value>, _environment: &mut Environment) -> Value {
    for value in args {
        print!("{value}");
    }
    Value::Unit
}

fn builtin_println(args: Vec<Value>, _environment: &mut Environment) -> Value {
    for value in args {
        println!("{value}");
    }
    Value::Unit
}

// number casts - will be removed when a proper cast system is developed

fn as_i8(args: Vec<Value>, _environment: &mut Environment) -> Value {
    if args.len() != 1 {
        unreachable!("semantic analyser missed arg count mismatch");
    }
    match args[0] {
        Value::Number(number) => match number {
            Number::I8(n) => Value::Number(Number::I8(n)),
            Number::I16(n) => Value::Number(Number::I8(n as i8)),
            Number::I32(n) => Value::Number(Number::I8(n as i8)),
            Number::I64(n) => Value::Number(Number::I8(n as i8)),
            Number::U8(n) => Value::Number(Number::I8(n as i8)),
            Number::U16(n) => Value::Number(Number::I8(n as i8)),
            Number::U32(n) => Value::Number(Number::I8(n as i8)),
            Number::U64(n) => Value::Number(Number::I8(n as i8)),
            Number::F32(n) => Value::Number(Number::I8(n as i8)),
            Number::F64(n) => Value::Number(Number::I8(n as i8)),
            Number::IntegerLiteral(n) => Value::Number(Number::I8(n as i8)),
            Number::FloatLiteral(n) => Value::Number(Number::I8(n as i8)),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => {
            unreachable!("type checker missed only nums casting")
        }
    }
}

fn as_i16(args: Vec<Value>, _environment: &mut Environment) -> Value {
    if args.len() != 1 {
        unreachable!("semantic analyser missed arg count mismatch");
    }
    match args[0] {
        Value::Number(number) => match number {
            Number::I8(n) => Value::Number(Number::I16(n as i16)),
            Number::I16(n) => Value::Number(Number::I16(n)),
            Number::I32(n) => Value::Number(Number::I16(n as i16)),
            Number::I64(n) => Value::Number(Number::I16(n as i16)),
            Number::U8(n) => Value::Number(Number::I16(n as i16)),
            Number::U16(n) => Value::Number(Number::I16(n as i16)),
            Number::U32(n) => Value::Number(Number::I16(n as i16)),
            Number::U64(n) => Value::Number(Number::I16(n as i16)),
            Number::F32(n) => Value::Number(Number::I16(n as i16)),
            Number::F64(n) => Value::Number(Number::I16(n as i16)),
            Number::IntegerLiteral(n) => Value::Number(Number::I16(n as i16)),
            Number::FloatLiteral(n) => Value::Number(Number::I16(n as i16)),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => {
            unreachable!("type checker missed only nums casting")
        }
    }
}

fn as_i32(args: Vec<Value>, _environment: &mut Environment) -> Value {
    if args.len() != 1 {
        unreachable!("semantic analyser missed arg count mismatch");
    }
    match args[0] {
        Value::Number(number) => match number {
            Number::I8(n) => Value::Number(Number::I32(n as i32)),
            Number::I16(n) => Value::Number(Number::I32(n as i32)),
            Number::I32(n) => Value::Number(Number::I32(n)),
            Number::I64(n) => Value::Number(Number::I32(n as i32)),
            Number::U8(n) => Value::Number(Number::I32(n as i32)),
            Number::U16(n) => Value::Number(Number::I32(n as i32)),
            Number::U32(n) => Value::Number(Number::I32(n as i32)),
            Number::U64(n) => Value::Number(Number::I32(n as i32)),
            Number::F32(n) => Value::Number(Number::I32(n as i32)),
            Number::F64(n) => Value::Number(Number::I32(n as i32)),
            Number::IntegerLiteral(n) => Value::Number(Number::I32(n as i32)),
            Number::FloatLiteral(n) => Value::Number(Number::I32(n as i32)),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => {
            unreachable!("type checker missed only nums casting")
        }
    }
}

fn as_i64(args: Vec<Value>, _environment: &mut Environment) -> Value {
    if args.len() != 1 {
        unreachable!("semantic analyser missed arg count mismatch");
    }
    match args[0] {
        Value::Number(number) => match number {
            Number::I8(n) => Value::Number(Number::I64(n as i64)),
            Number::I16(n) => Value::Number(Number::I64(n as i64)),
            Number::I32(n) => Value::Number(Number::I64(n as i64)),
            Number::I64(n) => Value::Number(Number::I64(n as i64)),
            Number::U8(n) => Value::Number(Number::I64(n as i64)),
            Number::U16(n) => Value::Number(Number::I64(n as i64)),
            Number::U32(n) => Value::Number(Number::I64(n as i64)),
            Number::U64(n) => Value::Number(Number::I64(n as i64)),
            Number::F32(n) => Value::Number(Number::I64(n as i64)),
            Number::F64(n) => Value::Number(Number::I64(n as i64)),
            Number::IntegerLiteral(n) => Value::Number(Number::I64(n as i64)),
            Number::FloatLiteral(n) => Value::Number(Number::I64(n as i64)),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => {
            unreachable!("type checker missed only nums casting")
        }
    }
}

fn as_u8(args: Vec<Value>, _environment: &mut Environment) -> Value {
    if args.len() != 1 {
        unreachable!("semantic analyser missed arg count mismatch");
    }
    match args[0] {
        Value::Number(number) => match number {
            Number::I8(n) => Value::Number(Number::U8(n as u8)),
            Number::I16(n) => Value::Number(Number::U8(n as u8)),
            Number::I32(n) => Value::Number(Number::U8(n as u8)),
            Number::I64(n) => Value::Number(Number::U8(n as u8)),
            Number::U8(n) => Value::Number(Number::U8(n)),
            Number::U16(n) => Value::Number(Number::U8(n as u8)),
            Number::U32(n) => Value::Number(Number::U8(n as u8)),
            Number::U64(n) => Value::Number(Number::U8(n as u8)),
            Number::F32(n) => Value::Number(Number::U8(n as u8)),
            Number::F64(n) => Value::Number(Number::U8(n as u8)),
            Number::IntegerLiteral(n) => Value::Number(Number::U8(n as u8)),
            Number::FloatLiteral(n) => Value::Number(Number::U8(n as u8)),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => {
            unreachable!("type checker missed only nums casting")
        }
    }
}

fn as_u16(args: Vec<Value>, _environment: &mut Environment) -> Value {
    if args.len() != 1 {
        unreachable!("semantic analyser missed arg count mismatch");
    }
    match args[0] {
        Value::Number(number) => match number {
            Number::I8(n) => Value::Number(Number::U16(n as u16)),
            Number::I16(n) => Value::Number(Number::U16(n as u16)),
            Number::I32(n) => Value::Number(Number::U16(n as u16)),
            Number::I64(n) => Value::Number(Number::U16(n as u16)),
            Number::U8(n) => Value::Number(Number::U16(n as u16)),
            Number::U16(n) => Value::Number(Number::U16(n)),
            Number::U32(n) => Value::Number(Number::U16(n as u16)),
            Number::U64(n) => Value::Number(Number::U16(n as u16)),
            Number::F32(n) => Value::Number(Number::U16(n as u16)),
            Number::F64(n) => Value::Number(Number::U16(n as u16)),
            Number::IntegerLiteral(n) => Value::Number(Number::U8(n as u8)),
            Number::FloatLiteral(n) => Value::Number(Number::U8(n as u8)),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => {
            unreachable!("type checker missed only nums casting")
        }
    }
}

fn as_u32(args: Vec<Value>, _environment: &mut Environment) -> Value {
    if args.len() != 1 {
        unreachable!("semantic analyser missed arg count mismatch");
    }
    match args[0] {
        Value::Number(number) => match number {
            Number::I8(n) => Value::Number(Number::U32(n as u32)),
            Number::I16(n) => Value::Number(Number::U32(n as u32)),
            Number::I32(n) => Value::Number(Number::U32(n as u32)),
            Number::I64(n) => Value::Number(Number::U32(n as u32)),
            Number::U8(n) => Value::Number(Number::U32(n as u32)),
            Number::U16(n) => Value::Number(Number::U32(n as u32)),
            Number::U32(n) => Value::Number(Number::U32(n)),
            Number::U64(n) => Value::Number(Number::U32(n as u32)),
            Number::F32(n) => Value::Number(Number::U32(n as u32)),
            Number::F64(n) => Value::Number(Number::U32(n as u32)),
            Number::IntegerLiteral(n) => Value::Number(Number::U32(n as u32)),
            Number::FloatLiteral(n) => Value::Number(Number::U32(n as u32)),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => {
            unreachable!("type checker missed only nums casting")
        }
    }
}

fn as_u64(args: Vec<Value>, _environment: &mut Environment) -> Value {
    if args.len() != 1 {
        unreachable!("semantic analyser missed arg count mismatch");
    }
    match args[0] {
        Value::Number(number) => match number {
            Number::I8(n) => Value::Number(Number::U64(n as u64)),
            Number::I16(n) => Value::Number(Number::U64(n as u64)),
            Number::I32(n) => Value::Number(Number::U64(n as u64)),
            Number::I64(n) => Value::Number(Number::U64(n as u64)),
            Number::U8(n) => Value::Number(Number::U64(n as u64)),
            Number::U16(n) => Value::Number(Number::U64(n as u64)),
            Number::U32(n) => Value::Number(Number::U64(n as u64)),
            Number::U64(n) => Value::Number(Number::U64(n)),
            Number::F32(n) => Value::Number(Number::U64(n as u64)),
            Number::F64(n) => Value::Number(Number::U64(n as u64)),
            Number::IntegerLiteral(n) => Value::Number(Number::U64(n as u64)),
            Number::FloatLiteral(n) => Value::Number(Number::U64(n as u64)),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => {
            unreachable!("type checker missed only nums casting")
        }
    }
}

fn as_f32(args: Vec<Value>, _environment: &mut Environment) -> Value {
    if args.len() != 1 {
        unreachable!("semantic analyser missed arg count mismatch");
    }
    match args[0] {
        Value::Number(number) => match number {
            Number::I8(n) => Value::Number(Number::F32(n as f32)),
            Number::I16(n) => Value::Number(Number::F32(n as f32)),
            Number::I32(n) => Value::Number(Number::F32(n as f32)),
            Number::I64(n) => Value::Number(Number::F32(n as f32)),
            Number::U8(n) => Value::Number(Number::F32(n as f32)),
            Number::U16(n) => Value::Number(Number::F32(n as f32)),
            Number::U32(n) => Value::Number(Number::F32(n as f32)),
            Number::U64(n) => Value::Number(Number::F32(n as f32)),
            Number::F32(n) => Value::Number(Number::F32(n)),
            Number::F64(n) => Value::Number(Number::F32(n as f32)),
            Number::IntegerLiteral(n) => Value::Number(Number::F32(n as f32)),
            Number::FloatLiteral(n) => Value::Number(Number::F32(n as f32)),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => {
            unreachable!("type checker missed only nums casting")
        }
    }
}

fn as_f64(args: Vec<Value>, _environment: &mut Environment) -> Value {
    if args.len() != 1 {
        unreachable!("semantic analyser missed arg count mismatch");
    }
    match args[0] {
        Value::Number(number) => match number {
            Number::I8(n) => Value::Number(Number::F64(n as f64)),
            Number::I16(n) => Value::Number(Number::F64(n as f64)),
            Number::I32(n) => Value::Number(Number::F64(n as f64)),
            Number::I64(n) => Value::Number(Number::F64(n as f64)),
            Number::U8(n) => Value::Number(Number::F64(n as f64)),
            Number::U16(n) => Value::Number(Number::F64(n as f64)),
            Number::U32(n) => Value::Number(Number::F64(n as f64)),
            Number::U64(n) => Value::Number(Number::F64(n as f64)),
            Number::F32(n) => Value::Number(Number::F64(n as f64)),
            Number::F64(n) => Value::Number(Number::F64(n)),
            Number::IntegerLiteral(n) => Value::Number(Number::F64(n as f64)),
            Number::FloatLiteral(n) => Value::Number(Number::F64(n as f64)),
        },
        Value::String(_) | Value::Bool(_) | Value::Unit => {
            unreachable!("type checker missed only nums casting")
        }
    }
}

fn cmd_input(args: Vec<Value>, _environment: &mut Environment) -> Value {
    if args.len() > 1 {
        unreachable!("semantic analyser missed arg count mismatch");
    }
    let user_input = input!("{}", args[0]);

    Value::String(user_input)
}
