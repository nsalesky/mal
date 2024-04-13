use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Integer(i64),
    Symbol(String),
    List(LinkedList<Expr>),
}
