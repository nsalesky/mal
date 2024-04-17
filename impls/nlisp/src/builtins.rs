use crate::env::Environment;
use crate::evaluator::RuntimeError;
use crate::types::Value;

// Arguments will be bound to `arg-1`, `arg-2`, ..., `arg-n`

pub fn add(env: &Environment) -> Result<Value, RuntimeError> {
    let val_a = env.lookup_symbol_err("arg-1")?;
    let val_b = env.lookup_symbol_err("arg-2")?;

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a + num_b))
        }
        _ => Err(RuntimeError::IncorrectType)
    }
}

pub fn sub(env: &Environment) -> Result<Value, RuntimeError> {
    let val_a = env.lookup_symbol_err("arg-1")?;
    let val_b = env.lookup_symbol_err("arg-2")?;

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a - num_b))
        }
        _ => Err(RuntimeError::IncorrectType)
    }
}

pub fn mul(env: &Environment) -> Result<Value, RuntimeError> {
    let val_a = env.lookup_symbol_err("arg-1")?;
    let val_b = env.lookup_symbol_err("arg-2")?;

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a * num_b))
        }
        _ => Err(RuntimeError::IncorrectType)
    }
}

pub fn div(env: &Environment) -> Result<Value, RuntimeError> {
    let val_a = env.lookup_symbol_err("arg-1")?;
    let val_b = env.lookup_symbol_err("arg-2")?;

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a / num_b))
        }
        _ => Err(RuntimeError::IncorrectType)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_add {
        use crate::env::Environment;
        use crate::types::Value;

        use super::*;

        #[test]
        fn test_good() {
            let env = Environment::with_values(vec![
                ("arg-1".to_string(), Value::Integer(4)),
                ("arg-2".to_string(), Value::Integer(-2)),
            ]);
            assert_eq!(Ok(Value::Integer(2)), add(&env));
        }

        #[test]
        fn test_unbound() {
            let env = Environment::default();
            assert_eq!(Err(RuntimeError::UnboundSymbol("arg-1".to_string())), add(&env));
        }

        #[test]
        fn test_wrong_types() {
            let env = Environment::with_values(vec![
                ("arg-1".to_string(), Value::Integer(4)),
                ("arg-2".to_string(), Value::String("foo".to_string())),
            ]);
            assert_eq!(Err(RuntimeError::IncorrectType), add(&env));
        }
    }
}