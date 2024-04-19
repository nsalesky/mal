use std::collections::VecDeque;
use std::hash::Hash;

use crate::env::Environment;
use crate::evaluator::{evaluate_expr, RuntimeError};
use crate::types::{Expr, Value};

fn assert_args_length<T>(args: &VecDeque<T>, expected_num_args: usize) -> Result<(), RuntimeError> {
    if args.len() != expected_num_args {
        return Err(RuntimeError::FunctionApplicationWrongNumberOfArgs {
            given: args.len(),
            expected: 2,
        });
    }
    Ok(())
}


pub fn add(env: &mut Environment, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_values, 2)?;

    let val_a = arg_values.pop_front().expect("val_a to be present");
    let val_b = arg_values.pop_front().expect("val_b to be present");

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a + num_b))
        }
        _ => Err(RuntimeError::IncorrectType)
    }
}

pub fn sub(env: &mut Environment, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_values, 2)?;

    let val_a = arg_values.pop_front().expect("val_a to be present");
    let val_b = arg_values.pop_front().expect("val_b to be present");

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a - num_b))
        }
        _ => Err(RuntimeError::IncorrectType)
    }
}

pub fn mul(env: &mut Environment, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_values, 2)?;

    let val_a = arg_values.pop_front().expect("val_a to be present");
    let val_b = arg_values.pop_front().expect("val_b to be present");

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a * num_b))
        }
        _ => Err(RuntimeError::IncorrectType)
    }
}

pub fn div(env: &mut Environment, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_values, 2)?;

    let val_a = arg_values.pop_front().expect("val_a to be present");
    let val_b = arg_values.pop_front().expect("val_b to be present");

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a / num_b))
        }
        _ => Err(RuntimeError::IncorrectType)
    }
}

pub fn def(env: &mut Environment, mut arg_exprs: VecDeque<Expr>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_exprs, 2)?;

    let expr_a = arg_exprs.pop_front().expect("expr_a to be present");
    let expr_b = arg_exprs.pop_front().expect("expr_b to be present");
    let val_b = evaluate_expr(expr_b, env)?;

    match expr_a {
        Expr::Symbol(id) => {
            env.insert_symbol(id, val_b.clone());
            Ok(val_b)
        }
        _ => Err(RuntimeError::ExpectedToBindSymbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_add {
        use crate::env::Environment;
        use crate::types::Value;

        use super::*;

        #[test]
        fn test_good() {
            let mut env = Environment::default();
            let values = VecDeque::from([
                Value::Integer(1),
                Value::Integer(2)
            ]);
            assert_eq!(Ok(Value::Integer(3)), add(&mut env, values));
        }

        #[test]
        fn test_wrong_num_args() {
            let mut env = Environment::default();
            let values = VecDeque::from([
                Value::Integer(1),
            ]);
            assert_eq!(Err(RuntimeError::FunctionApplicationWrongNumberOfArgs { expected: 2, given: 1 }), add(&mut env, values));
        }

        #[test]
        fn test_wrong_types() {
            let mut env = Environment::default();
            let values = VecDeque::from([
                Value::Integer(1),
                Value::Symbol("foo".to_string())
            ]);
            assert_eq!(Err(RuntimeError::IncorrectType), add(&mut env, values));
        }
    }

    mod test_def {
        use super::*;

        #[test]
        fn test_good() {
            let mut env = Environment::default();
            let exprs = VecDeque::from([
                Expr::Symbol("foo".to_string()),
                Expr::Integer(4)
            ]);
            assert_eq!(Ok(Value::Integer(4)), def(&mut env, exprs));
            assert_eq!(Some(Value::Integer(4)), env.lookup_symbol("foo"));
        }

        #[test]
        fn test_wrong_num_args() {
            let mut env = Environment::default();
            let exprs = VecDeque::from([
                Expr::Symbol("foo".to_string()),
            ]);
            assert_eq!(Err(RuntimeError::FunctionApplicationWrongNumberOfArgs { expected: 2, given: 1 }), def(&mut env, exprs));
        }

        #[test]
        fn test_identifier_not_symbol() {
            let mut env = Environment::default();
            let exprs = VecDeque::from([
                Expr::Integer(3),
                Expr::Integer(4)
            ]);
            assert_eq!(Err(RuntimeError::ExpectedToBindSymbol), def(&mut env, exprs));
        }
    }
}