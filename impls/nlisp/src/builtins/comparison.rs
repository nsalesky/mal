use std::collections::VecDeque;

use crate::builtins::{assert_args_length, run_to_closure};
use crate::Environment;
use crate::evaluator::{RuntimeError, TypeError};
use crate::types::{FunctionBody, Value};

pub fn insert_functions(env: &mut Environment) {
    env.insert_symbol("=".to_string(), Value::Function(
        FunctionBody::BuiltinValues(eq)
    ));
    env.insert_symbol("<".to_string(), Value::Function(
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

pub fn insert_core_closures(into_env: &mut Environment, closure_env: &mut Environment) {
    into_env.insert_symbol("not".to_string(), run_to_closure("(fn* (x) (if x false true))", closure_env));
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

#[cfg(test)]
mod tests {
    use super::*;

    mod test_eq {
        use super::*;

        #[test]
        fn test_equal() {
            let mut env = Environment::default();
            let args = VecDeque::from([
                Value::Integer(3), Value::Integer(3)
            ]);
            assert_eq!(Ok(Value::Boolean(true)), eq(&mut env, args));
        }

        #[test]
        fn test_not_equal() {
            let mut env = Environment::default();
            let args = VecDeque::from([
                Value::String("abc".to_string()), Value::String("foo".to_string())
            ]);
            assert_eq!(Ok(Value::Boolean(false)), eq(&mut env, args));
        }

        #[test]
        fn test_seq_coercion() {
            let mut env = Environment::default();
            let args = VecDeque::from([
                Value::List(rpds::List::from_iter([Value::Integer(1), Value::Symbol("foo".to_string())])),
                Value::Vector(rpds::Vector::from_iter([Value::Integer(1), Value::Symbol("foo".to_string())]))
            ]);
            assert_eq!(Ok(Value::Boolean(true)), eq(&mut env, args));
        }

        #[test]
        fn test_different_types() {
            let mut env = Environment::default();
            let args = VecDeque::from([
                Value::String("abc".to_string()), Value::Integer(4)
            ]);
            assert_eq!(Ok(Value::Boolean(false)), eq(&mut env, args));
        }
    }
}