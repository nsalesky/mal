use std::collections::VecDeque;

use crate::Environment;
use crate::evaluator::RuntimeError;

mod arithmetic;
mod special_forms;
mod list;
mod io;
mod comparison;

pub fn insert_core_functions(env: &mut Environment) {
    arithmetic::insert_functions(env);
    special_forms::insert_functions(env);
    list::insert_functions(env);
    io::insert_functions(env);
    comparison::insert_functions(env);
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
