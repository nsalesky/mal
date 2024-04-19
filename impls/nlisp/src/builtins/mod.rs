use std::collections::VecDeque;
use std::hash::Hash;

pub use arithmetic::{add, div, mul, sub};
pub use special_forms::def;

use crate::env::Environment;
use crate::evaluator::RuntimeError;
use crate::types::{Expr, Value};

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
