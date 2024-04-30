use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::builtins::assert_args_length;
use crate::Env;
use crate::evaluator::{RuntimeError, TypeError};
use crate::types::{FunctionBody, Value};

pub fn insert_functions(env: &Env) {
    env.insert("atom".to_string(), Value::Function(
        FunctionBody::BuiltinValues(atom)
    ));
    env.insert("deref".to_string(), Value::Function(
        FunctionBody::BuiltinValues(deref)
    ));
}

fn atom(_env: &Env, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;

    let val = args.pop_front().expect("atom to have one argument");
    Ok(Value::Atom(Rc::new(RefCell::new(val))))
}

fn deref(_env: &Env, mut args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    assert_args_length(&args, 1)?;

    let arg = args.pop_front().expect("deref to have one argument");
    match arg {
        Value::Atom(val) => {
            Ok(val.borrow().clone())
        }
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc))
    }
}