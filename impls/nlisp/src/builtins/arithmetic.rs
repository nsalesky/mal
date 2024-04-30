use std::collections::VecDeque;

use crate::builtins::assert_args_length;
use crate::Env;
use crate::evaluator::{RuntimeError, TypeError};
use crate::types::{FunctionBody, Value};

pub fn insert_functions(env: &Env) {
    env.insert("+".to_string(), Value::Function(
        FunctionBody::BuiltinValues(add)
    ));
    env.insert("-".to_string(), Value::Function(
        FunctionBody::BuiltinValues(sub)
    ));
    env.insert("*".to_string(), Value::Function(
        FunctionBody::BuiltinValues(mul)
    ));
    env.insert("/".to_string(), Value::Function(
        FunctionBody::BuiltinValues(div)
    ));
}

fn add(_env: &Env, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
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

fn sub(_env: &Env, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
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

fn mul(_env: &Env, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
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

fn div(_env: &Env, mut arg_values: VecDeque<Value>) -> Result<Value, RuntimeError> {
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

        use crate::env::Env;
        use crate::evaluator::RuntimeError;
        use crate::types::Value;

        use super::*;

        #[test]
        fn test_good() {
            let env = Env::default();
            let values = VecDeque::from([
                Value::Integer(1),
                Value::Integer(2)
            ]);
            assert_eq!(Ok(Value::Integer(3)), add(&env, values));
        }

        #[test]
        fn test_wrong_num_args() {
            let env = Env::default();
            let values = VecDeque::from([
                Value::Integer(1),
            ]);
            assert_eq!(Err(RuntimeError::FunctionApplicationWrongNumberOfArgs { expected: 2, given: 1 }), add(&env, values));
        }

        #[test]
        fn test_wrong_types() {
            let env = Env::default();
            let values = VecDeque::from([
                Value::Integer(1),
                Value::Symbol("foo".to_string())
            ]);
            assert_eq!(Err(RuntimeError::IncorrectType(TypeError::Misc)), add(&env, values));
        }
    }
}