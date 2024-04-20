use std::collections::VecDeque;

use crate::builtins::assert_args_length;
use crate::Environment;
use crate::evaluator::RuntimeError;
use crate::types::{FunctionBody, Value};

pub fn insert_functions(env: &mut Environment) {
    env.insert_symbol("prn".to_string(), Value::Function(
        FunctionBody::BuiltinValues(prn)
    ));
}

fn prn(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;
    let arg = args.pop_front().expect("prn to have one argument");
    println!("{}", arg);
    Ok(Value::Nil)
}