use std::collections::VecDeque;

use crate::{builtins, Environment};
use crate::evaluator::{evaluate_expr, RuntimeError};
use crate::types::{Expr, Value};

pub fn def(env: &mut Environment, mut arg_exprs: VecDeque<Expr>) -> Result<Value, RuntimeError> {
    builtins::assert_args_length(&arg_exprs, 2)?;

    let expr_a = arg_exprs.pop_front().expect("expr_a to be present");
    let expr_b = arg_exprs.pop_front().expect("expr_b to be present");
    let val_b = evaluate_expr(expr_b, env)?;

    match expr_a {
        Expr::Symbol(id) => {
            env.insert_symbol(id, val_b.clone());
            Ok(val_b)
        }
        _ => Err(RuntimeError::ExpectedToBindSymbol)
    }
}

#[cfg(test)]
mod tests {
    use crate::builtins::*;

    mod test_def {
        use crate::builtins::special_forms::def;

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
}
