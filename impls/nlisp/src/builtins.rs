use crate::env::Environment;
use crate::evaluator::RuntimeError;
use crate::types::Value;

// Arguments will be bound to `arg-a` and `arg-b`
pub fn add(env: &Environment) -> Result<Value, RuntimeError> {
    let val_a = env.lookup_symbol_err("arg-a")?;
    let val_b = env.lookup_symbol_err("arg-b")?;

    match (val_a, val_b) {
        (Value::Integer(num_a), Value::Integer(num_b)) => {
            Ok(Value::Integer(num_a + num_b))
        }
        _ => Err(RuntimeError::IncorrectType)
    }
}