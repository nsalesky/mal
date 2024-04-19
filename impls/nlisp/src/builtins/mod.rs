use std::collections::VecDeque;

pub use arithmetic::{add, div, mul, sub};
pub use special_forms::{def, fn_f, let_f};

use crate::evaluator::RuntimeError;

mod arithmetic;
mod special_forms;

fn assert_args_length<T>(args: &VecDeque<T>, expected_num_args: usize) -> Result<(), RuntimeError> {
    if args.len() != expected_num_args {
        return Err(RuntimeError::FunctionApplicationWrongNumberOfArgs {
            given: args.len(),
            expected: 2,
        });
    }
    Ok(())
}
