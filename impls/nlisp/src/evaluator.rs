use std::collections::{HashMap, LinkedList, VecDeque};

use thiserror::Error;

use crate::env::Environment;
use crate::evaluator::RuntimeError::HashError;
use crate::parser::ParseError;
use crate::types::{Expr, FunctionBody, HashableValue, Value};

#[derive(Error, Debug, PartialEq)]
pub enum TypeError {
    #[error("miscellaneous type error. This should eventually be replaced with more specific errors")]
    Misc
}

#[derive(Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("parse error: `{0}`")]
    ParseError(#[from] ParseError),

    #[error("attempted to hash an unhashable value: `{0}`")]
    HashError(Value),

    #[error("symbol '{0}' not found")]
    UnboundSymbol(String),

    #[error("attempted to apply an expression that did not evaluate to a function")]
    CannotApplyNonFunction,

    #[error("attempted to apply a function with the wrong number of arguments. Given {given} but expected {expected} args")]
    FunctionApplicationWrongNumberOfArgs { given: usize, expected: usize },

    #[error("expected to bind to a symbol value, but expression evaluated to a different type")]
    ExpectedToBindSymbol,

    #[error("expression evaluated to the wrong type: {0}")]
    IncorrectType(#[from] TypeError),

    #[error("encountered a let binding identifier with no corresponding assignment")]
    UnmatchedLetBindingID,

    #[error("a miscellaneous error. These should eventually be replaced with more specific errors")]
    Misc,
}

pub fn evaluate_expr(expr: Expr, env: &mut Environment) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Integer(num) => Ok(Value::Integer(num)),
        Expr::String(s) => Ok(Value::String(s)),
        Expr::Keyword(s) => Ok(Value::Keyword(s)),
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
            let mut list_expr_iter = list_exprs.iter();
            match list_expr_iter.next() {
                Some(expr) => match evaluate_expr(expr.clone(), env) {
                    Ok(Value::Function(func_body)) => {
                        apply_function(func_body,
                                       list_expr_iter
                                           .map(|expr| expr.clone())
                                           .collect(),
                                       env)
                    }
                    Ok(_) => Err(RuntimeError::CannotApplyNonFunction),
                    Err(e) => Err(e),
                }
                None => Ok(Value::List(LinkedList::new()))
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

fn apply_function(function_body: FunctionBody, arg_exprs: VecDeque<Expr>, env: &mut Environment) -> Result<Value, RuntimeError> {
    match function_body {
        FunctionBody::BuiltinExpressions(func_pointer) => func_pointer(env, arg_exprs),
        FunctionBody::BuiltinValues(func_pointer) => {
            let mut arg_values = VecDeque::with_capacity(arg_exprs.len());
            for arg_expr in arg_exprs {
                arg_values.push_back(evaluate_expr(arg_expr, env)?);
            }
            func_pointer(env, arg_values)
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
        env.insert_symbol("foo".to_string(), Value::Integer(3));
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