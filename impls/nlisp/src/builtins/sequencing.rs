use std::collections::VecDeque;

use crate::builtins::{assert_args_length_at_least, assert_args_length_between};
use crate::Env;
use crate::evaluator::{evaluate_expr, RuntimeError};
use crate::types::{Expr, FunctionBody, Value};

pub fn insert_functions(env: &Env) {
    env.insert("if".to_string(), Value::Function(
        FunctionBody::BuiltinExpressions(if_f)
    ));
    env.insert("do".to_string(), Value::Function(
        FunctionBody::BuiltinExpressions(do_f)
    ));
}

fn if_f(env: &Env, mut args: VecDeque<Expr>) -> Result<Value, RuntimeError> {
    assert_args_length_between(&args, 2, 3)?;

    let guard_expr = args.pop_front().expect("if to have a guard expression");
    let then_expr = args.pop_front().expect("if to have a then expression");
    let else_expr = args.pop_front();

    let guard_value = match evaluate_expr(guard_expr, env)? {
        Value::Boolean(bool) => bool,
        Value::Nil => false,
        _ => true,
    };

    if guard_value {
        evaluate_expr(then_expr, env)
    } else if let Some(else_expr) = else_expr {
        evaluate_expr(else_expr, env)
    } else {
        Ok(Value::Nil)
    }
}

fn do_f(env: &Env, args: VecDeque<Expr>) -> Result<Value, RuntimeError> {
    // NOTE: I technically could have just taken args as values, but I wanted to make sure they get executed in the right order
    // if I change the way I bind arguments
    assert_args_length_at_least(&args, 1)?;
    let mut last_value = None;
    for arg in args {
        last_value = Some(evaluate_expr(arg, env)?);
    }
    Ok(last_value.expect("the last expression in a `do` sequence to evaluate to a value"))
}