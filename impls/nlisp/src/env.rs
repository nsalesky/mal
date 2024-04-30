use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::builtins;
use crate::evaluator::RuntimeError;
use crate::types::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct EnvData {
    map: RefCell<HashMap<String, Value>>,
    outer: Option<Env>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Env(Rc<EnvData>);

impl Default for Env {
    fn default() -> Self {
        let new_env = Env::with_core_functions();
        builtins::insert_core_closures(&new_env, &Env::with_core_functions());
        new_env
    }
}

impl Env {
    pub fn with_core_functions() -> Self {
        let map = RefCell::new(HashMap::new());
        let new_env = Env(Rc::new(EnvData { map, outer: None }));
        builtins::insert_core_functions(&new_env);
        new_env
    }

    pub fn with_map(map: HashMap<String, Value>) -> Self {
        Env(Rc::new(EnvData {
            map: RefCell::new(map),
            outer: None,
        }))
    }

    pub fn create_child_env(&self) -> Self {
        Env(Rc::new(EnvData {
            map: RefCell::new(HashMap::new()),
            outer: Some(self.clone()),
        }))
    }

    pub fn lookup(&self, symbol_name: &str) -> Option<Value> {
        if let Some(val) = self.0.map.borrow().get(symbol_name) {
            return Some(val.clone());
        }

        if let Some(outer) = &self.0.outer {
            return outer.lookup(symbol_name);
        }

        None
    }

    pub fn lookup_err(&self, symbol_name: &str) -> Result<Value, RuntimeError> {
        match self.lookup(symbol_name) {
            Some(val) => Ok(val),
            None => Err(RuntimeError::UnboundSymbol(symbol_name.to_string()))
        }
    }

    pub fn with_symbol(&self, symbol_name: String, val: Value) -> Self {
        Env(Rc::new(EnvData {
            outer: Some(self.clone()),
            map: RefCell::new(HashMap::from([(symbol_name, val)])),
        }))
    }

    pub fn insert(&self, symbol_name: String, val: Value) {
        self.0.map.borrow_mut().insert(symbol_name, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_symbol() {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), Value::Integer(4));

        let env = Env::with_map(map);
        assert_eq!(Some(Value::Integer(4)), env.lookup("foo"));
        assert_eq!(None, env.lookup("bar"));
    }

    #[test]
    fn test_lookup_symbol_err() {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), Value::Integer(4));

        let env = Env::with_map(map);
        assert_eq!(Ok(Value::Integer(4)), env.lookup_err("foo"));
        assert_eq!(Err(RuntimeError::UnboundSymbol("bar".to_string())), env.lookup_err("bar"));
    }

    #[test]
    fn test_with_symbol() {
        let env = Env::default();
        assert_eq!(None, env.lookup("foo"));
        let new_env = env.with_symbol("foo".to_string(), Value::String("hello".to_string()));
        assert_eq!(Some(Value::String("hello".to_string())), new_env.lookup("foo"));
    }

    #[test]
    fn test_insert_symbol() {
        let env = Env::default();
        assert_eq!(None, env.lookup("foo"));
        env.insert("foo".to_string(), Value::String("hello".to_string()));
        assert_eq!(Some(Value::String("hello".to_string())), env.lookup("foo"));
    }

    #[test]
    fn test_with_values() {
        let env = Env::with_map(HashMap::new());
        env.insert("foo".to_string(), Value::Symbol("hello".to_string()));
        env.insert("bar".to_string(), Value::Integer(0));
        assert_eq!(Some(Value::Symbol("hello".to_string())), env.lookup("foo"));
        assert_eq!(Some(Value::Integer(0)), env.lookup("bar"));
    }
}