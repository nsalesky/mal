use std::collections::VecDeque;

use crate::builtins::assert_args_length;
use crate::Env;
use crate::evaluator::RuntimeError;
use crate::types::{FunctionBody, Value};

pub fn insert_functions(env: &Env) {
    env.insert("list".to_string(), Value::Function(
        FunctionBody::BuiltinValues(list_f)
    ));
    env.insert("list?".to_string(), Value::Function(
        FunctionBody::BuiltinValues(list_p)
    ));
    env.insert("empty?".to_string(), Value::Function(
        FunctionBody::BuiltinValues(empty_p)
    ));
    env.insert("count".to_string(), Value::Function(
        FunctionBody::BuiltinValues(count)
    ));
}

/// The builtin definition for `list`
fn list_f(_env: &Env, args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    let mut ret_list = rpds::List::new();
    for arg in args.into_iter().rev() {
        ret_list.push_front_mut(arg);
    }
    Ok(Value::List(ret_list))
}

/// The builtin definition for `list?`
fn list_p(_env: &Env, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;
    let arg = args.pop_front().expect("list? to have an argument");
    match arg {
        Value::List(_) => Ok(Value::Boolean(true)),
        _ => Ok(Value::Boolean(false))
    }
}

/// The builtin definition for `empty?`
fn empty_p(_env: &Env, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;
    let arg = args.pop_front().expect("empty? to have an argument");
    let seq = arg.to_seq()?;
    Ok(Value::Boolean(seq.len() == 0))
}

/// The builtin definition for `count`
fn count(_env: &Env, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;
    let arg = args.pop_front().expect("count to have an argument");
    let seq = arg.to_seq()?;
    Ok(Value::Integer(seq.len() as i64))
}
