use rpds::HashTrieMap;

use crate::builtins;
use crate::evaluator::RuntimeError;
use crate::types::{FunctionBody, Value};

#[derive(Clone)]
pub struct Environment {
    symbol_values: HashTrieMap<String, Value>,
}

impl Default for Environment {
    fn default() -> Self {
        let mut default_env = HashTrieMap::new();

        default_env.insert_mut("+".to_string(), Value::Function {
            arg_names: vec!["arg-a".to_string(), "arg-b".to_string()],
            body: Box::new(FunctionBody::Builtin(builtins::add)),
        });

        Self {
            symbol_values: default_env
        }
    }
}

impl Environment {
    pub fn lookup_symbol(&self, symbol_name: &str) -> Option<Value> {
        match self.symbol_values.get(symbol_name) {
            Some(val) => Some(val.clone()),
            None => None
        }
    }

    pub fn lookup_symbol_err(&self, symbol_name: &str) -> Result<Value, RuntimeError> {
        match self.symbol_values.get(symbol_name) {
            Some(val) => Ok(val.clone()),
            None => Err(RuntimeError::UnboundSymbol(symbol_name.to_string()))
        }
    }

    pub fn insert_symbol(&self, symbol_name: String, val: Value) -> Self {
        Self {
            symbol_values: self.symbol_values.insert(symbol_name, val),
        }
    }

    pub fn insert_symbols<T, V>(&self, pairs: T) -> Self
        where T: IntoIterator<Item=(String, Value)> {
        let mut new_symbol_values = self.symbol_values.clone();

        for (symbol_name, val) in pairs {
            new_symbol_values.insert_mut(symbol_name, val);
        }

        Self {
            symbol_values: new_symbol_values
        }
    }

    pub fn insert_global_symbol(&mut self, symbol_name: String, val: Value) {
        self.symbol_values.insert_mut(symbol_name, val);
    }
}