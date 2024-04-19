use std::collections::VecDeque;

use crate::builtins::assert_args_length;
use crate::Environment;
use crate::evaluator::{RuntimeError, TypeError};
use crate::types::Value;

pub fn add(_env: &mut Environment, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_values, 2)?;

    let val_a = arg_values.pop_front().expect("val_a to be present");
    let val_b = arg_values.pop_front().expect("val_b to be present");

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a + num_b))
        }
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc))
    }
}

pub fn sub(_env: &mut Environment, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_values, 2)?;

    let val_a = arg_values.pop_front().expect("val_a to be present");
    let val_b = arg_values.pop_front().expect("val_b to be present");

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a - num_b))
        }
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc))
    }
}

pub fn mul(_env: &mut Environment, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_values, 2)?;

    let val_a = arg_values.pop_front().expect("val_a to be present");
    let val_b = arg_values.pop_front().expect("val_b to be present");

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a * num_b))
        }
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc))
    }
}

pub fn div(_env: &mut Environment, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_values, 2)?;

    let val_a = arg_values.pop_front().expect("val_a to be present");
    let val_b = arg_values.pop_front().expect("val_b to be present");

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a / num_b))
        }
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_add {
        use std::collections::VecDeque;

        use crate::env::Environment;
        use crate::evaluator::RuntimeError;
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
            assert_eq!(Err(RuntimeError::IncorrectType(TypeError::Misc)), add(&mut env, values));
        }
    }
}