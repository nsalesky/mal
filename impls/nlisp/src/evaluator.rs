use std::collections::{HashMap, LinkedList};

use thiserror::Error;

use crate::env::Environment;
use crate::evaluator::RuntimeError::HashError;
use crate::parser::ParseError;
use crate::types::{Expr, FunctionBody, HashableValue, Value};

#[derive(Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("parse error: `{0}`")]
    ParseError(#[from] ParseError),

    #[error("attempted to hash an unhashable value: `{0}`")]
    HashError(Value),

    #[error("attempted to access an unbound symbol: `{0}`")]
    UnboundSymbol(String),

    #[error("attempted to apply an expression that did not evaluate to a function")]
    CannotApplyNonFunction,

    #[error("attempted to apply a function with the wrong number of arguments. Given {0} but expected {1} args")]
    FunctionApplicationWrongNumberOfArgs(u32, u32),

    #[error("expression evaluated to the wrong type")]
    IncorrectType,
}

pub fn evaluate_expr(expr: Expr, env: &mut Environment) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Integer(num) => Ok(Value::Integer(num)),
        Expr::String(s) => Ok(Value::String(s)),
        Expr::Symbol(s) => {
            match env.lookup_symbol(s.as_str()) {
                Some(val) => Ok(val),
                None => Err(RuntimeError::UnboundSymbol(s)),
            }
        }
        Expr::Nil => Ok(Value::Nil),
        Expr::Boolean(b) => Ok(Value::Boolean(b)),
        Expr::Quote(_) => todo!(),
        Expr::Quasiquote(_) => todo!(),
        Expr::Unquote(_) => todo!(),
        Expr::SpliceUnquote(_) => todo!(),
        Expr::List(list_exprs) => {
            let mut list_values = Vec::with_capacity(list_exprs.len());
            for list_expr in list_exprs {
                list_values.push(evaluate_expr(list_expr, env)?);
            }
            let expected_arg_count = list_values.len() - 1;
            let mut list_values = list_values.iter_mut();

            match list_values.next() {
                Some(Value::Function { arg_names, body }) => {
                    if arg_names.len() != expected_arg_count {
                        return Err(RuntimeError::FunctionApplicationWrongNumberOfArgs(expected_arg_count as u32, arg_names.len() as u32));
                    }

                    // Create a new environment with all of the argument names bound to their values
                    let mut new_env = env.to_owned();
                    for (arg_name, binding_value) in arg_names.iter().zip(list_values) {
                        new_env.insert_global_symbol(arg_name.to_owned(), binding_value.to_owned());
                    }

                    match **body {
                        FunctionBody::Builtin(fn_pointer) => {
                            fn_pointer(&new_env)
                        }
                    }
                }
                None => Ok(Value::List(LinkedList::new())),
                _ => Err(RuntimeError::CannotApplyNonFunction)
            }
        }
        Expr::Vector(v) => {
            let mut ret_vec = Vec::with_capacity(v.len());
            for expr_elem in v {
                ret_vec.push(evaluate_expr(expr_elem, env)?);
            }
            Ok(Value::Vector(ret_vec))
        }
        Expr::HashMap(hashmap_pairs) => {
            let mut ret_hashmap = HashMap::new();
            for (key_expr, value_expr) in hashmap_pairs {
                let key_value = evaluate_expr(key_expr, env)?;
                let value_value = evaluate_expr(value_expr, env)?;
                let key_hash: HashableValue = key_value.clone().try_into().map_err(|_| HashError(key_value))?;
                ret_hashmap.insert(key_hash, value_value);
            }

            Ok(Value::HashMap(ret_hashmap))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_integer() {
        let mut env = Environment::default();
        assert_eq!(Ok(Value::Integer(3)), evaluate_expr(Expr::Integer(3), &mut env));
    }

    #[test]
    fn test_evaluate_string() {
        let mut env = Environment::default();
        assert_eq!(Ok(Value::String("hello".to_string())), evaluate_expr(Expr::String("hello".to_string()), &mut env));
    }

    #[test]
    fn test_evaluate_boolean() {
        let mut env = Environment::default();
        assert_eq!(Ok(Value::Boolean(true)), evaluate_expr(Expr::Boolean(true), &mut env));
        assert_eq!(Ok(Value::Boolean(false)), evaluate_expr(Expr::Boolean(false), &mut env));
    }

    #[test]
    fn test_evaluate_symbol() {
        let mut env = Environment::default();
        assert_eq!(Err(RuntimeError::UnboundSymbol("foo".to_string())), evaluate_expr(Expr::Symbol("foo".to_string()), &mut env));
        env.insert_global_symbol("foo".to_string(), Value::Integer(3));
        assert_eq!(Ok(Value::Integer(3)), evaluate_expr(Expr::Symbol("foo".to_string()), &mut env));
    }

    #[test]
    fn test_apply_builtin_function() {
        let mut env = Environment::default();
        let my_expr = Expr::List(LinkedList::from([
            Expr::Symbol("+".to_string()),
            Expr::Integer(1),
            Expr::Integer(2),
        ]));
        assert_eq!(Ok(Value::Integer(3)), evaluate_expr(my_expr, &mut env));
    }
}