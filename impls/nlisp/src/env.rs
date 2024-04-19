use rpds::HashTrieMap;

use crate::builtins;
use crate::evaluator::RuntimeError;
use crate::types::{FunctionBody, Value};

#[derive(Clone)]
pub struct Environment {
    env: HashTrieMap<String, Value>,
}

impl Default for Environment {
    fn default() -> Self {
        let mut default_env = HashTrieMap::new();

        default_env.insert_mut("+".to_string(), Value::Function(
            FunctionBody::BuiltinValues(builtins::add)
        ));
        default_env.insert_mut("-".to_string(), Value::Function(
            FunctionBody::BuiltinValues(builtins::sub)
        ));
        default_env.insert_mut("*".to_string(), Value::Function(
            FunctionBody::BuiltinValues(builtins::mul)
        ));
        default_env.insert_mut("/".to_string(), Value::Function(
            FunctionBody::BuiltinValues(builtins::div)
        ));
        default_env.insert_mut("def!".to_string(), Value::Function(
            FunctionBody::BuiltinExpressions(builtins::def)
        ));

        Self {
            env: default_env
        }
    }
}

impl Environment {
    pub fn with_values<T>(values: T) -> Self
        where T: IntoIterator<Item=(String, Value)>
    {
        let mut env = Environment::default();
        for (symbol, binding) in values {
            env.insert_symbol(symbol, binding);
        }
        env
    }

    pub fn lookup_symbol(&self, symbol_name: &str) -> Option<Value> {
        match self.env.get(symbol_name) {
            Some(val) => Some(val.clone()),
            None => None
        }
    }

    pub fn lookup_symbol_err(&self, symbol_name: &str) -> Result<Value, RuntimeError> {
        match self.env.get(symbol_name) {
            Some(val) => Ok(val.clone()),
            None => Err(RuntimeError::UnboundSymbol(symbol_name.to_string()))
        }
    }

    pub fn with_symbol(&self, symbol_name: String, val: Value) -> Self {
        Self {
            env: self.env.insert(symbol_name, val),
        }
    }

    pub fn insert_symbol(&mut self, symbol_name: String, val: Value) {
        self.env.insert_mut(symbol_name, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_symbol() {
        let mut inner_env = HashTrieMap::new();
        inner_env.insert_mut("foo".to_string(), Value::Integer(4));

        let env = Environment { env: inner_env };
        assert_eq!(Some(Value::Integer(4)), env.lookup_symbol("foo"));
        assert_eq!(None, env.lookup_symbol("bar"));
    }

    #[test]
    fn test_lookup_symbol_err() {
        let mut inner_env = HashTrieMap::new();
        inner_env.insert_mut("foo".to_string(), Value::Integer(4));

        let env = Environment { env: inner_env };
        assert_eq!(Ok(Value::Integer(4)), env.lookup_symbol_err("foo"));
        assert_eq!(Err(RuntimeError::UnboundSymbol("bar".to_string())), env.lookup_symbol_err("bar"));
    }

    #[test]
    fn test_with_symbol() {
        let env = Environment::default();
        assert_eq!(None, env.lookup_symbol("foo"));
        let new_env = env.with_symbol("foo".to_string(), Value::String("hello".to_string()));
        assert_eq!(Some(Value::String("hello".to_string())), new_env.lookup_symbol("foo"));
    }

    #[test]
    fn test_insert_symbol() {
        let mut env = Environment::default();
        assert_eq!(None, env.lookup_symbol("foo"));
        env.insert_symbol("foo".to_string(), Value::String("hello".to_string()));
        assert_eq!(Some(Value::String("hello".to_string())), env.lookup_symbol("foo"));
    }

    #[test]
    fn test_with_values() {
        let env = Environment::with_values(vec![
            ("foo".to_string(), Value::Symbol("hello".to_string())),
            ("bar".to_string(), Value::Integer(0)),
        ]);
        assert_eq!(Some(Value::Symbol("hello".to_string())), env.lookup_symbol("foo"));
        assert_eq!(Some(Value::Integer(0)), env.lookup_symbol("bar"));
    }
}