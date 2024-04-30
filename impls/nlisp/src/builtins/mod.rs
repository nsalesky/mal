use std::collections::VecDeque;

use crate::Env;
use crate::evaluator::{evaluate_expr, RuntimeError};
use crate::parser::parse_text_to_expression;
use crate::types::Value;

mod arithmetic;
mod special_forms;
mod list;
mod io;
mod comparison;
mod sequencing;
mod string;
mod atoms;

pub fn insert_core_functions(env: &Env) {
    arithmetic::insert_functions(env);
    special_forms::insert_functions(env);
    list::insert_functions(env);
    io::insert_functions(env);
    comparison::insert_functions(env);
    sequencing::insert_functions(env);
    string::insert_functions(env);
    atoms::insert_functions(env);
}

pub fn insert_core_closures(into_env: &Env, closure_env: &Env) {
    comparison::insert_core_closures(into_env, closure_env);
}

fn run_to_closure(expr_src: &str, env: &Env) -> Value {
    let expr = parse_text_to_expression(expr_src).expect("expr_src to be valid source code");
    let value = evaluate_expr(expr, env).expect("expr_src to evaluate to a value");
    match value {
        Value::Function(function_body) => Value::Function(function_body),
        _ => panic!("expected expr_src to evaluate to a function")
    }
}

fn assert_args_length<T>(args: &VecDeque<T>, expected_num_args: usize) -> Result<(), RuntimeError> {
    if args.len() != expected_num_args {
        return Err(RuntimeError::FunctionApplicationWrongNumberOfArgs {
            given: args.len(),
            expected: 2,
        });
    }
    Ok(())
}

fn assert_args_length_at_least<T>(args: &VecDeque<T>, expected_min_num_args: usize) -> Result<(), RuntimeError> {
    if args.len() < expected_min_num_args {
        return Err(RuntimeError::FunctionApplicationWrongNumberOfArgs {
            given: args.len(),
            expected: expected_min_num_args,
        });
    }
    Ok(())
}

fn assert_args_length_between<T>(args: &VecDeque<T>, min_num_args: usize, max_num_args: usize) -> Result<(), RuntimeError> {
    if args.len() < min_num_args || args.len() > max_num_args {
        return Err(RuntimeError::FunctionApplicationWrongNumberOfArgs {
            given: args.len(),
            expected: min_num_args,
        });
    }
    Ok(())
}
