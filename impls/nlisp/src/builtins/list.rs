use std::collections::{LinkedList, VecDeque};

use crate::builtins::assert_args_length;
use crate::Environment;
use crate::evaluator::{RuntimeError, TypeError};
use crate::types::Value;

/// The builtin definition for `list`
pub fn list_f(_env: &mut Environment, args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    let mut ret_list = LinkedList::new();
    for arg in args {
        ret_list.push_back(arg);
    }
    Ok(Value::List(ret_list))
}

/// The builtin definition for `list?`
pub fn list_p(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;
    let arg = args.pop_front().expect("list? to have an argument");
    match arg {
        Value::List(_) => Ok(Value::Boolean(true)),
        _ => Ok(Value::Boolean(false))
    }
}

/// The builtin definition for `empty?`
pub fn empty_p(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;
    let arg = args.pop_front().expect("empty? to have an argument");
    match arg {
        Value::List(elems) => Ok(Value::Boolean(elems.is_empty())),
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc)),
    }
}

/// The builtin definition for `count`
pub fn count(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;
    let arg = args.pop_front().expect("count to have an argument");
    match arg {
        Value::List(elems) => Ok(Value::Integer(elems.len() as i64)),
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc)),
    }
}
