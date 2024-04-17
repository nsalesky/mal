use std::collections::HashMap;

use thiserror::Error;

use crate::env::Environment;
use crate::evaluator::RuntimeError::HashError;
use crate::parser::ParseError;
use crate::types::{Expr, HashableValue, Value};

#[derive(Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("parse error: `{0}`")]
    ParseError(#[from] ParseError),

    #[error("attempted to hash an unhashable value: `{0}`")]
    HashError(Value),

    #[error("attempted to access an unbound symbol: `{0}`")]
    UnboundSymbol(String),
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
            todo!()
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
}