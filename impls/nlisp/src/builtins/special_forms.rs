use std::collections::{LinkedList, VecDeque};

use itertools::Itertools;

use crate::builtins::assert_args_length;
use crate::Env;
use crate::evaluator::{evaluate_expr, RuntimeError, TypeError};
use crate::types::{Expr, FunctionBody, Value};

pub fn insert_functions(env: &Env) {
    env.insert("def!".to_string(), Value::Function(
        FunctionBody::BuiltinExpressions(def)
    ));
    env.insert("let*".to_string(), Value::Function(
        FunctionBody::BuiltinExpressions(let_f)
    ));
    env.insert("fn*".to_string(), Value::Function(
        FunctionBody::BuiltinExpressions(fn_f)
    ));
}

fn def(env: &Env, mut arg_exprs: VecDeque<Expr>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_exprs, 2)?;

    let expr_a = arg_exprs.pop_front().expect("id to be present");

    match expr_a {
        Expr::Symbol(id) => {
            let assignment_expr = arg_exprs.pop_front().expect("assignment to be present");
            let assignment_val = evaluate_expr(assignment_expr, env)?;
            env.insert(id, assignment_val.clone());
            Ok(assignment_val)
        }
        _ => Err(RuntimeError::ExpectedToBindSymbol)
    }
}

fn create_environment_for_bindings<T>(base_env: &Env, binding_exprs: T) -> Result<Env, RuntimeError>
    where T: Iterator<Item=Expr> {
    let new_env = base_env.create_child_env();
    for mut chunk_exprs in &binding_exprs.chunks(2) {
        let binding_expr = chunk_exprs.next().expect("let binding chunk to have a binding");
        let assignment_expr = chunk_exprs.next().expect("let binding chunk to have an assignment");
        let assignment_val = evaluate_expr(assignment_expr.clone(), &new_env)?;

        match binding_expr {
            Expr::Symbol(s) => {
                new_env.insert(s.to_string(), assignment_val);
            }
            _ => return Err(RuntimeError::ExpectedToBindSymbol)
        }
    }
    Ok(new_env)
}

fn let_f(env: &Env, mut arg_exprs: VecDeque<Expr>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_exprs, 2)?;

    let bindings_expr = arg_exprs.pop_front().expect("binding to be present");
    let body_expr = arg_exprs.pop_front().expect("body to be present");

    let new_env = match bindings_expr {
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

    evaluate_expr(body_expr, &new_env)
}

fn fn_f(env: &Env, mut arg_exprs: VecDeque<Expr>) -> Result<Value, RuntimeError> {
    assert_args_length(&arg_exprs, 2)?;

    let param_list_expr = arg_exprs.pop_front().expect("parameter list to be present");
    let body_expr = arg_exprs.pop_front().expect("function body to be present");

    let param_name_exprs = {
        match param_list_expr {
            Expr::List(elems) => elems,
            Expr::Vector(elems) => LinkedList::from_iter(elems),
            _ => return Err(RuntimeError::ExpectedToBindSymbol),
        }
    };

    let mut param_names = Vec::with_capacity(param_name_exprs.len());
    let mut expecting_variadic = false;
    let mut variadic_param = None;
    for elem in param_name_exprs {
        match elem {
            Expr::Symbol(param_name) => {
                if param_name == "&" {
                    if expecting_variadic {
                        return Err(RuntimeError::Misc); // Can't have duplicate &
                    }

                    // Consume the next param as variadic
                    expecting_variadic = true;
                } else if expecting_variadic && variadic_param.is_some() {
                    return Err(RuntimeError::Misc); // Can only have one variadic param
                } else if expecting_variadic && variadic_param.is_none() {
                    variadic_param = Some(param_name);
                } else {
                    param_names.push(param_name)
                }
            }
            _ => return Err(RuntimeError::ExpectedToBindSymbol),
        }
    }
    if expecting_variadic && variadic_param.is_none() {
        return Err(RuntimeError::Misc); // If you use &, you need to give a name to the variadic params
    }

    Ok(Value::Function(FunctionBody::Closure {
        closed_env: env.clone(),
        params: param_names,
        variadic_param,
        body: body_expr,
    }))
}

#[cfg(test)]
mod tests {
    use crate::builtins::*;

    use super::*;

    mod test_def {
        use crate::builtins::special_forms::def;
        use crate::Env;

        use super::*;

        #[test]
        fn test_good() {
            let env = Env::default();
            let exprs = VecDeque::from([
                Expr::Symbol("foo".to_string()),
                Expr::Integer(4)
            ]);
            assert_eq!(Ok(Value::Integer(4)), def(&env, exprs));
            assert_eq!(Some(Value::Integer(4)), env.lookup("foo"));
        }

        #[test]
        fn test_wrong_num_args() {
            let env = Env::default();
            let exprs = VecDeque::from([
                Expr::Symbol("foo".to_string()),
            ]);
            assert_eq!(Err(RuntimeError::FunctionApplicationWrongNumberOfArgs { expected: 2, given: 1 }), def(&env, exprs));
        }

        #[test]
        fn test_identifier_not_symbol() {
            let env = Env::default();
            let exprs = VecDeque::from([
                Expr::Integer(3),
                Expr::Integer(4)
            ]);
            assert_eq!(Err(RuntimeError::ExpectedToBindSymbol), def(&env, exprs));
        }
    }

    mod test_let_f {
        use std::collections::LinkedList;

        use super::*;

        #[test]
        fn test_good() {
            let env = Env::default();
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
            assert_eq!(Ok(Value::Integer(12)), let_f(&env, exprs));
        }

        #[test]
        fn test_wrong_num_args() {
            let env = Env::default();
            let exprs = VecDeque::from([
                Expr::List(LinkedList::new()),
            ]);
            assert_eq!(Err(RuntimeError::FunctionApplicationWrongNumberOfArgs { given: 1, expected: 2 }),
                       let_f(&env, exprs));
        }

        #[test]
        fn test_unmatched_pair() {
            let env = Env::default();
            let exprs = VecDeque::from([
                Expr::List(LinkedList::from([
                    Expr::Symbol("a".to_string()),
                ])),
                Expr::Integer(3),
            ]);
            assert_eq!(Err(RuntimeError::UnmatchedLetBindingID), let_f(&env, exprs));
        }

        #[test]
        fn test_binding_id_not_symbol() {
            let env = Env::default();
            let exprs = VecDeque::from([
                Expr::List(LinkedList::from([
                    Expr::Integer(5),
                    Expr::String("hello world".to_string()),
                ])),
                Expr::Integer(3),
            ]);
            assert_eq!(Err(RuntimeError::ExpectedToBindSymbol), let_f(&env, exprs));
        }
    }

    mod test_fn_f {
        use std::collections::{LinkedList, VecDeque};

        use crate::Env;
        use crate::evaluator::RuntimeError;
        use crate::types::{Expr, FunctionBody, Value};

        use super::*;

        #[test]
        fn test_good() {
            let env = Env::default();
            let exprs = VecDeque::from([
                Expr::List(LinkedList::from([
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("y".to_string()),
                ])),
                Expr::List(LinkedList::from([
                    Expr::Symbol("+".to_string()),
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("y".to_string()),
                ]))
            ]);
            let expected_value = Value::Function(FunctionBody::Closure {
                closed_env: env.clone(),
                params: vec!["x".to_string(), "y".to_string()],
                variadic_param: None,
                body: Expr::List(LinkedList::from([
                    Expr::Symbol("+".to_string()),
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("y".to_string()),
                ])),
            });

            assert_eq!(Ok(expected_value), fn_f(&env, exprs));
        }

        #[test]
        fn test_wrong_num_args() {
            let env = Env::default();
            let exprs = VecDeque::from([
                Expr::List(LinkedList::from([
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("y".to_string()),
                ])),
            ]);

            assert_eq!(Err(RuntimeError::FunctionApplicationWrongNumberOfArgs { expected: 2, given: 1 }),
                       fn_f(&env, exprs));
        }

        #[test]
        fn test_params_not_list() {
            let env = Env::default();
            let exprs = VecDeque::from([
                Expr::String("foo".to_string()),
                Expr::Integer(2)
            ]);

            assert_eq!(Err(RuntimeError::ExpectedToBindSymbol),
                       fn_f(&env, exprs));
        }

        #[test]
        fn test_param_not_symbol() {
            let env = Env::default();
            let exprs = VecDeque::from([
                Expr::List(LinkedList::from([
                    Expr::Symbol("x".to_string()),
                    Expr::Integer(3),
                ])),
                Expr::Integer(2)
            ]);

            assert_eq!(Err(RuntimeError::ExpectedToBindSymbol),
                       fn_f(&env, exprs));
        }

        #[test]
        fn test_variadic_parameters() {
            let env = Env::default();
            let exprs = VecDeque::from([
                Expr::List(LinkedList::from([
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("&".to_string()),
                    Expr::Symbol("rest".to_string()),
                ])),
                Expr::Symbol("rest".to_string()),
            ]);
            let expected_value = Value::Function(FunctionBody::Closure {
                closed_env: env.clone(),
                params: vec!["x".to_string()],
                variadic_param: Some("rest".to_string()),
                body: Expr::Symbol("rest".to_string()),
            });

            assert_eq!(Ok(expected_value), fn_f(&env, exprs));
        }
    }
}
