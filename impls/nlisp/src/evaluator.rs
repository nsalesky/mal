use std::collections::VecDeque;

use thiserror::Error;

use crate::env::Env;
use crate::evaluator::RuntimeError::HashError;
use crate::parser::ParseError;
use crate::types::{Expr, FunctionBody, HashableValue, Value};

#[derive(Error, Debug, PartialEq)]
pub enum TypeError {
    #[error("miscellaneous type error. This should eventually be replaced with more specific errors")]
    Misc,

    #[error("expected a sequence, but given something that cannot be converted")]
    NotASeq,
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

pub fn evaluate_expr(expr: Expr, env: &Env) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Integer(num) => Ok(Value::Integer(num)),
        Expr::String(s) => Ok(Value::String(s)),
        Expr::Keyword(s) => Ok(Value::Keyword(s)),
        Expr::Symbol(s) => {
            match env.lookup(s.as_str()) {
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
                                           .cloned()
                                           .collect(),
                                       env)
                    }
                    Ok(_) => Err(RuntimeError::CannotApplyNonFunction),
                    Err(e) => Err(e),
                }
                None => Ok(Value::List(rpds::List::new()))
            }
        }
        Expr::Vector(v) => {
            let mut ret_vec = rpds::Vector::new();
            for expr_elem in v {
                ret_vec.push_back_mut(evaluate_expr(expr_elem, env)?);
            }
            Ok(Value::Vector(ret_vec))
        }
        Expr::HashMap(hashmap_pairs) => {
            let mut ret_hashmap = rpds::HashTrieMap::new();
            for (key_expr, value_expr) in hashmap_pairs {
                let key_value = evaluate_expr(key_expr, env)?;
                let value_value = evaluate_expr(value_expr, env)?;
                let key_hash: HashableValue = key_value.clone().try_into().map_err(|_| HashError(key_value))?;
                ret_hashmap.insert_mut(key_hash, value_value);
            }

            Ok(Value::HashMap(ret_hashmap))
        }
    }
}

fn apply_function(function_body: FunctionBody, arg_exprs: VecDeque<Expr>, env: &Env) -> Result<Value, RuntimeError> {
    match function_body {
        FunctionBody::BuiltinExpressions(func_pointer) => func_pointer(env, arg_exprs),
        FunctionBody::BuiltinValues(func_pointer) => {
            let mut arg_values = VecDeque::with_capacity(arg_exprs.len());
            for arg_expr in arg_exprs {
                arg_values.push_back(evaluate_expr(arg_expr, env)?);
            }
            func_pointer(env, arg_values)
        }
        FunctionBody::Closure { closed_env, params, variadic_param, body } => {
            if arg_exprs.len() < params.len() || (arg_exprs.len() > params.len() && variadic_param.is_none()) {
                return Err(RuntimeError::FunctionApplicationWrongNumberOfArgs { expected: params.len(), given: arg_exprs.len() });
            }

            let mut args_iter = arg_exprs.into_iter();

            // Consume the named arguments
            let mut arg_values = Vec::with_capacity(params.len());
            for _ in 0..(params.len()) {
                arg_values.push(evaluate_expr(
                    args_iter.next().expect("arg to be present"),
                    env)?);
            }

            let new_env = closed_env.create_child_env();
            for (param_name, arg_value) in params.iter().zip(arg_values) {
                new_env.insert(param_name.to_string(), arg_value);
            }

            // Consume the remaining arguments
            if let Some(variadic_param_name) = variadic_param {
                let mut variadic_values = rpds::List::new();
                for arg_expr in args_iter.rev() {
                    variadic_values.push_front_mut(evaluate_expr(arg_expr, env)?);
                }
                new_env.insert(variadic_param_name, Value::List(variadic_values));
            }

            evaluate_expr(body, &new_env)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use super::*;

    #[test]
    fn test_evaluate_integer() {
        let env = Env::default();
        assert_eq!(Ok(Value::Integer(3)), evaluate_expr(Expr::Integer(3), &env));
    }

    #[test]
    fn test_evaluate_string() {
        let env = Env::default();
        assert_eq!(Ok(Value::String("hello".to_string())), evaluate_expr(Expr::String("hello".to_string()), &env));
    }

    #[test]
    fn test_evaluate_boolean() {
        let env = Env::default();
        assert_eq!(Ok(Value::Boolean(true)), evaluate_expr(Expr::Boolean(true), &env));
        assert_eq!(Ok(Value::Boolean(false)), evaluate_expr(Expr::Boolean(false), &env));
    }

    #[test]
    fn test_evaluate_symbol() {
        let env = Env::default();
        assert_eq!(Err(RuntimeError::UnboundSymbol("foo".to_string())), evaluate_expr(Expr::Symbol("foo".to_string()), &env));
        env.insert("foo".to_string(), Value::Integer(3));
        assert_eq!(Ok(Value::Integer(3)), evaluate_expr(Expr::Symbol("foo".to_string()), &env));
    }

    #[test]
    fn test_apply_builtin_function() {
        let env = Env::default();
        let my_expr = Expr::List(LinkedList::from([
            Expr::Symbol("+".to_string()),
            Expr::Integer(1),
            Expr::Integer(2),
        ]));
        assert_eq!(Ok(Value::Integer(3)), evaluate_expr(my_expr, &env));
    }

    #[test]
    fn test_apply_closure() {
        let env = Env::default();
        let my_expr = Expr::List(LinkedList::from([
            Expr::List(LinkedList::from([
                Expr::Symbol("fn*".to_string()),
                Expr::List(LinkedList::from([
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("y".to_string()),
                ])),
                Expr::List(LinkedList::from([
                    Expr::Symbol("+".to_string()),
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("y".to_string()),
                ]))])),
            Expr::Integer(4),
            Expr::Integer(8),
        ]));

        assert_eq!(Ok(Value::Integer(12)), evaluate_expr(my_expr, &env));
    }
}