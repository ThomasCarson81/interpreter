use crate::{Environment, RuntimeError, Value};

pub fn register_all(environment: &mut Environment) {
    environment.define_builtin_func(String::from("print"), builtin_print);
    environment.define_builtin_func(String::from("println"), builtin_println);
}

fn builtin_print(args: Vec<Value>, _environment: &mut Environment) -> Result<Value, RuntimeError> {
    for value in args {
        print!("{:?}", value);
    }
    Ok(Value::Unit)
}

fn builtin_println(
    args: Vec<Value>,
    _environment: &mut Environment,
) -> Result<Value, RuntimeError> {
    for value in args {
        println!("{:?}", value);
    }
    Ok(Value::Unit)
}
