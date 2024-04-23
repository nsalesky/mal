use std::collections::VecDeque;

use itertools::Itertools;

use crate::Environment;
use crate::evaluator::RuntimeError;
use crate::printer::Printable;
use crate::types::{FunctionBody, Value};

pub fn insert_functions(env: &mut Environment) {
    env.insert_symbol("pr-str".to_string(), Value::Function(
        FunctionBody::BuiltinValues(pr_str)
    ));
    env.insert_symbol("str".to_string(), Value::Function(
        FunctionBody::BuiltinValues(str)
    ));
}

fn pr_str(_env: &mut Environment, args: VecDeque<Value>) -> Result<Value, RuntimeError> {
    let result = args.into_iter()
        .map(|value| value.print_value(true))
        .join(" ");
    Ok(Value::String(result))
}

fn str(_env: &mut Environment, args: VecDeque<Value>) -> Result<Value, RuntimeError> {
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
        assert_eq!(Ok(Value::String("\"foo\" \"bar\"".to_string())), pr_str(&mut Environment::default(), args));
    }
}