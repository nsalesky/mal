use std::collections::VecDeque;

use crate::builtins::assert_args_length;
use crate::Environment;
use crate::evaluator::{RuntimeError, TypeError};
use crate::types::{FunctionBody, Value};

pub fn insert_functions(env: &mut Environment) {
    env.insert_symbol("=".to_string(), Value::Function(
        FunctionBody::BuiltinValues(eq)
    ));
    env.insert_symbol("<=".to_string(), Value::Function(
        FunctionBody::BuiltinValues(lt)
    ));
    env.insert_symbol("<=".to_string(), Value::Function(
        FunctionBody::BuiltinValues(lte)
    ));
    env.insert_symbol(">".to_string(), Value::Function(
        FunctionBody::BuiltinValues(gt)
    ));
    env.insert_symbol(">=".to_string(), Value::Function(
        FunctionBody::BuiltinValues(gte)
    ));
}

fn eq(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 2)?;
    let lhs = args.pop_front().expect("= to have a LHS argument");
    let rhs = args.pop_front().expect("= to have a RHS argument");

    Ok(Value::Boolean(lhs == rhs))
}

fn lt(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 2)?;
    let lhs = args.pop_front().expect("< to have a LHS argument");
    let rhs = args.pop_front().expect("< to have a RHS argument");

    match (lhs, rhs) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs < rhs)),
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc))
    }
}

fn lte(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 2)?;
    let lhs = args.pop_front().expect("<= to have a LHS argument");
    let rhs = args.pop_front().expect("<= to have a RHS argument");

    match (lhs, rhs) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs <= rhs)),
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc))
    }
}

fn gt(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 2)?;
    let lhs = args.pop_front().expect("> to have a LHS argument");
    let rhs = args.pop_front().expect("> to have a RHS argument");

    match (lhs, rhs) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs > rhs)),
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc))
    }
}

fn gte(_env: &mut Environment, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 2)?;
    let lhs = args.pop_front().expect(">= to have a LHS argument");
    let rhs = args.pop_front().expect(">= to have a RHS argument");

    match (lhs, rhs) {
        (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(lhs >= rhs)),
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc))
    }
}
