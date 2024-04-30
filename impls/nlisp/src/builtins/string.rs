use std::collections::VecDeque;

use itertools::Itertools;

use crate::Env;
use crate::evaluator::RuntimeError;
use crate::printer::Printable;
use crate::types::{FunctionBody, Value};

pub fn insert_functions(env: &Env) {
    env.insert("pr-str".to_string(), Value::Function(
        FunctionBody::BuiltinValues(pr_str)
    ));
    env.insert("str".to_string(), Value::Function(
        FunctionBody::BuiltinValues(str)
    ));
}

fn pr_str(_env: &Env, args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    let result = args.into_iter()
        .map(|value| value.print_value(true))
        .join(" ");
    Ok(Value::String(result))
}

fn str(_env: &Env, args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    let result = args.into_iter()
        .map(|value| value.print_value(false))
        .join("");
    Ok(Value::String(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pr_str() {
        let args = VecDeque::from([
            Value::String("foo".to_string()),
            Value::String("bar".to_string()),
        ]);
        assert_eq!(Ok(Value::String("\"foo\" \"bar\"".to_string())), pr_str(&Env::default(), args));
    }
}