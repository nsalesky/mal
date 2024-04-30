use std::collections::VecDeque;

use itertools::Itertools;

use crate::Env;
use crate::evaluator::RuntimeError;
use crate::printer::Printable;
use crate::types::{FunctionBody, Value};

pub fn insert_functions(env: &Env) {
    env.insert("prn".to_string(), Value::Function(
        FunctionBody::BuiltinValues(prn)
    ));
    env.insert("println".to_string(), Value::Function(
        FunctionBody::BuiltinValues(println)
    ));
}


fn prn(_env: &Env, args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    let output = args.into_iter()
        .map(|value| value.print_value(true))
        .join(" ");
    println!("{}", output);
    Ok(Value::Nil)
}

fn println(_env: &Env, args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    let output = args.into_iter()
        .map(|value| value.print_value(false))
        .join(" ");
    println!("{}", output);
    Ok(Value::Nil)
}
