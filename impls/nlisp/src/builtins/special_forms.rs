use std::collections::VecDeque;

use itertools::Itertools;

use crate::builtins::assert_args_length;
use crate::Environment;
use crate::evaluator::{evaluate_expr, RuntimeError, TypeError};
use crate::types::{Expr, Value};

pub fn def(env: &mut Environment, mut arg_exprs: VecDeque<Expr>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_exprs, 2)?;

    let expr_a = arg_exprs.pop_front().expect("id to be present");
    let expr_b = arg_exprs.pop_front().expect("assignment to be present");
    let val_b = evaluate_expr(expr_b, env)?;

    match expr_a {
        Expr::Symbol(id) => {
            env.insert_symbol(id, val_b.clone());
            Ok(val_b)
        }
        _ => Err(RuntimeError::ExpectedToBindSymbol)
    }
}

fn create_environment_for_bindings<T>(base_env: &Environment, binding_exprs: T) -> Result<Environment, RuntimeError>
    where T: Iterator<Item=Expr> {
    let mut new_env = base_env.clone();
    for mut chunk_exprs in &binding_exprs.chunks(2) {
        let binding_expr = chunk_exprs.next().expect("let binding chunk to have a binding");
        let assignment_expr = chunk_exprs.next().expect("let binding chunk to have an assignment");
        let assignment_val = evaluate_expr(assignment_expr.clone(), &mut new_env)?;

        match binding_expr {
            Expr::Symbol(s) => {
                new_env.insert_symbol(s.to_string(), assignment_val);
            }
            _ => return Err(RuntimeError::ExpectedToBindSymbol)
        }
    }
    Ok(new_env)
}

pub fn let_f(env: &mut Environment, mut arg_exprs: VecDeque<Expr>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_exprs, 2)?;

    let bindings_expr = arg_exprs.pop_front().expect("binding to be present");
    let body_expr = arg_exprs.pop_front().expect("body to be present");

    let mut new_env = match bindings_expr {
        Expr::List(binding_exprs) => {
            if binding_exprs.len() % 2 != 0 {
                Err(RuntimeError::UnmatchedLetBindingID)
            } else {
                create_environment_for_bindings(env, binding_exprs.into_iter())
            }
        }
        Expr::Vector(binding_exprs) => {
            if binding_exprs.len() % 2 != 0 {
                Err(RuntimeError::UnmatchedLetBindingID)
            } else {
                create_environment_for_bindings(env, binding_exprs.into_iter())
            }
        }
        _ => Err(RuntimeError::IncorrectType(TypeError::Misc))
    }?;

    evaluate_expr(body_expr, &mut new_env)
}

#[cfg(test)]
mod tests {
    use crate::builtins::*;

    use super::*;

    mod test_def {
        use crate::builtins::special_forms::def;
        use crate::Environment;

        use super::*;

        #[test]
        fn test_good() {
            let mut env = Environment::default();
            let exprs = VecDeque::from([
                Expr::Symbol("foo".to_string()),
                Expr::Integer(4)
            ]);
            assert_eq!(Ok(Value::Integer(4)), def(&mut env, exprs));
            assert_eq!(Some(Value::Integer(4)), env.lookup_symbol("foo"));
        }

        #[test]
        fn test_wrong_num_args() {
            let mut env = Environment::default();
            let exprs = VecDeque::from([
                Expr::Symbol("foo".to_string()),
            ]);
            assert_eq!(Err(RuntimeError::FunctionApplicationWrongNumberOfArgs { expected: 2, given: 1 }), def(&mut env, exprs));
        }

        #[test]
        fn test_identifier_not_symbol() {
            let mut env = Environment::default();
            let exprs = VecDeque::from([
                Expr::Integer(3),
                Expr::Integer(4)
            ]);
            assert_eq!(Err(RuntimeError::ExpectedToBindSymbol), def(&mut env, exprs));
        }
    }

    mod test_let_f {
        use std::collections::LinkedList;

        use super::*;

        #[test]
        fn test_good() {
            let mut env = Environment::default();
            let exprs = VecDeque::from([
                Expr::List(LinkedList::from([
                    Expr::Symbol("a".to_string()),
                    Expr::Integer(3),
                    Expr::Symbol("b".to_string()),
                    Expr::List(LinkedList::from([
                        Expr::Symbol("+".to_string()),
                        Expr::Symbol("a".to_string()),
                        Expr::Integer(1)
                    ])),
                ])),
                Expr::List(LinkedList::from([
                    Expr::Symbol("*".to_string()),
                    Expr::Symbol("a".to_string()),
                    Expr::Symbol("b".to_string()),
                ]))
            ]);
            assert_eq!(Ok(Value::Integer(12)), let_f(&mut env, exprs));
        }

        #[test]
        fn test_wrong_num_args() {
            let mut env = Environment::default();
            let exprs = VecDeque::from([
                Expr::List(LinkedList::new()),
            ]);
            assert_eq!(Err(RuntimeError::FunctionApplicationWrongNumberOfArgs { given: 1, expected: 2 }),
                       let_f(&mut env, exprs));
        }

        #[test]
        fn test_unmatched_pair() {
            let mut env = Environment::default();
            let exprs = VecDeque::from([
                Expr::List(LinkedList::from([
                    Expr::Symbol("a".to_string()),
                ])),
                Expr::Integer(3),
            ]);
            assert_eq!(Err(RuntimeError::UnmatchedLetBindingID), let_f(&mut env, exprs));
        }

        #[test]
        fn test_binding_id_not_symbol() {
            let mut env = Environment::default();
            let exprs = VecDeque::from([
                Expr::List(LinkedList::from([
                    Expr::Integer(5),
                    Expr::String("hello world".to_string()),
                ])),
                Expr::Integer(3),
            ]);
            assert_eq!(Err(RuntimeError::ExpectedToBindSymbol), let_f(&mut env, exprs));
        }
    }
}
