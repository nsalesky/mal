use std::collections::{HashMap, LinkedList};

use thiserror::Error;

use crate::env::Environment;
use crate::evaluator::RuntimeError;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Integer(i64),
    String(String),
    Symbol(String),
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
    Builtin(fn(&Environment) -> Result<Value, RuntimeError>),
    // TODO: user defined functions
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    String(String),
    Symbol(String),
    Boolean(bool),
    List(LinkedList<Value>),
    Vector(Vec<Value>),
    HashMap(HashMap<HashableValue, Value>),
    Function { arg_names: Vec<String>, body: Box<FunctionBody> },
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashableValue {
    Integer(i64),
    String(String),
    Symbol(String),
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
            Value::Symbol(s) => Ok(HashableValue::Symbol(s)),
            _ => Err(HashValueError::UnhashableValue(self))
        }
    }
}
