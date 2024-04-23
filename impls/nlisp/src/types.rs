use std::collections::{LinkedList, VecDeque};

use thiserror::Error;

use crate::env::Environment;
use crate::evaluator::{RuntimeError, TypeError};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Integer(i64),
    String(String),
    Symbol(String),
    Keyword(String),
    Nil,
    Boolean(bool),
    Quote(Box<Expr>),
    Quasiquote(Box<Expr>),
    Unquote(Box<Expr>),
    SpliceUnquote(Box<Expr>),
    List(LinkedList<Expr>),
    Vector(Vec<Expr>),
    HashMap(Vec<(Expr, Expr)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionBody {
    BuiltinValues(fn(&mut Environment, VecDeque<Value>) -> Result<Value, RuntimeError>),
    BuiltinExpressions(fn(&mut Environment, VecDeque<Expr>) -> Result<Value, RuntimeError>),
    Closure { closed_env: Environment, params: Vec<String>, variadic_param: Option<String>, body: Expr },
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    String(String),
    Symbol(String),
    Keyword(String),
    Boolean(bool),
    List(rpds::List<Value>),
    Vector(rpds::Vector<Value>),
    HashMap(rpds::HashTrieMap<HashableValue, Value>),
    Function(FunctionBody),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashableValue {
    Integer(i64),
    String(String),
    Keyword(String),
}

#[derive(Error, Debug)]
pub enum HashValueError {
    #[error("Value `{0}` cannot be hashed")]
    UnhashableValue(Value),
}

impl TryInto<HashableValue> for Value {
    type Error = HashValueError;

    fn try_into(self) -> Result<HashableValue, Self::Error> {
        match self {
            Value::Integer(num) => Ok(HashableValue::Integer(num)),
            Value::String(s) => Ok(HashableValue::String(s)),
            Value::Keyword(s) => Ok(HashableValue::Keyword(s)),
            _ => Err(HashValueError::UnhashableValue(self))
        }
    }
}

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Integer(num_l), Value::Integer(num_r)) => num_l == num_r,
            (Value::String(str_l), Value::String(str_r)) => str_l == str_r,
            (Value::Symbol(sym_l), Value::Symbol(sym_r)) => sym_l == sym_r,
            (Value::Keyword(kwd_l), Value::Keyword(kwd_r)) => kwd_l == kwd_r,
            (Value::Boolean(bool_l), Value::Boolean(bool_r)) => bool_l == bool_r,
            (Value::HashMap(map_l), Value::HashMap(map_r)) => map_l == map_r,
            (Value::Function(func_l), Value::Function(func_r)) => func_l == func_r,
            // Special case: Nil is treated as a sequence for everything except equality
            (Value::Nil, Value::Nil) => true,
            (Value::Nil, _) => false,
            (_, Value::Nil) => false,
            _ => {
                // Otherwise, if the types don't match, or they are lists/vectors,
                // convert to sequence before comparing
                let seq_lhs = self.clone().to_seq();
                let seq_rhs = other.clone().to_seq();
                match (seq_lhs, seq_rhs) {
                    (Ok(seq_lhs), Ok(seq_rhs)) => seq_lhs == seq_rhs,
                    _ => false
                }
            }
        }
    }
}

impl Value {
    pub fn to_seq(self) -> Result<rpds::List<Value>, RuntimeError> {
        match self {
            Value::List(values) => Ok(values),
            Value::Vector(values) => Ok(values.into_iter().cloned().collect()),
            Value::Nil => Ok(rpds::List::new()),
            _ => Err(RuntimeError::IncorrectType(TypeError::NotASeq)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nil_equality() {
        assert_eq!(Value::Nil, Value::Nil);
        assert_ne!(Value::Nil, Value::List(rpds::List::new()));
        assert_ne!(Value::Vector(rpds::Vector::new()), Value::Nil);
    }

    #[test]
    fn test_to_seq() {
        assert_eq!(Ok(rpds::List::new()), Value::Nil.to_seq());

        let list_elems = [Value::Integer(1), Value::String("foo".to_string())];
        assert_eq!(Ok(rpds::List::from_iter(list_elems.clone())), Value::List(rpds::List::from_iter(list_elems.clone())).to_seq());
        assert_eq!(Ok(rpds::List::from_iter(list_elems.clone())), Value::Vector(rpds::Vector::from_iter(list_elems.clone())).to_seq());
    }
}