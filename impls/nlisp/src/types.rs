use std::collections::{HashMap, LinkedList, VecDeque};

use thiserror::Error;

use crate::env::Environment;
use crate::evaluator::RuntimeError;

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
    // TODO: user defined functions
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    String(String),
    Symbol(String),
    Keyword(String),
    Boolean(bool),
    List(LinkedList<Value>),
    Vector(Vec<Value>),
    HashMap(HashMap<HashableValue, Value>),
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
