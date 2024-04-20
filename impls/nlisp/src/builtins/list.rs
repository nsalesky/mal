use std::collections::{LinkedList, VecDeque};

use crate::builtins::assert_args_length;
use crate::Environment;
use crate::evaluator::RuntimeError;
use crate::types::{FunctionBody, Value};

pub fn insert_functions(env: &mut Environment) {
    env.insert_symbol("list".to_string(), Value::Function(
        FunctionBody::BuiltinValues(list_f)
    ));
    env.insert_symbol("list?".to_string(), Value::Function(
        FunctionBody::BuiltinValues(list_p)
    ));
    env.insert_symbol("empty?".to_string(), Value::Function(
        FunctionBody::BuiltinValues(empty_p)
    ));
    env.insert_symbol("count".to_string(), Value::Function(
        FunctionBody::BuiltinValues(count)
    ));
}

/// The builtin definition for `list`
fn list_f(_env: &mut Environment, args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    let mut ret_list = LinkedList::new();
    for arg in args {
        ret_list.push_back(arg);
    }
    Ok(Value::List(ret_list))
}

/// The builtin definition for `list?`
fn list_p(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;
    let arg = args.pop_front().expect("list? to have an argument");
    match arg {
        Value::List(_) => Ok(Value::Boolean(true)),
        _ => Ok(Value::Boolean(false))
    }
}

/// The builtin definition for `empty?`
fn empty_p(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;
    let arg = args.pop_front().expect("empty? to have an argument");
    let seq = arg.to_seq()?;
    Ok(Value::Boolean(seq.count() == 0))
}

/// The builtin definition for `count`
fn count(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;
    let arg = args.pop_front().expect("count to have an argument");
    let seq = arg.to_seq()?;
    Ok(Value::Integer(seq.count() as i64))
}
