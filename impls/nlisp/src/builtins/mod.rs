use std::collections::VecDeque;

use crate::Environment;
use crate::evaluator::RuntimeError;

mod arithmetic;
mod special_forms;
mod list;
mod io;
mod comparison;
mod sequencing;
mod string;

pub fn insert_core_functions(env: &mut Environment) {
    arithmetic::insert_functions(env);
    special_forms::insert_functions(env);
    list::insert_functions(env);
    io::insert_functions(env);
    comparison::insert_functions(env);
    sequencing::insert_functions(env);
    string::insert_functions(env);
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
