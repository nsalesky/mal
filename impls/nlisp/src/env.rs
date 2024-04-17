use rpds::HashTrieMap;

use crate::types::Value;

pub struct Environment {
    symbol_values: HashTrieMap<String, Value>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            symbol_values: HashTrieMap::new()
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

    pub fn insert_symbol(&self, symbol_name: String, val: Value) -> Environment {
        Environment {
            symbol_values: self.symbol_values.insert(symbol_name, val),
        }
    }

    pub fn insert_global_symbol(&mut self, symbol_name: String, val: Value) {
        self.symbol_values.insert_mut(symbol_name, val);
    }
}