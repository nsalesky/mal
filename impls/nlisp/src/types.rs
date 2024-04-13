use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Integer(i64),
    String(String),
    Symbol(String),
    Nil,
    True,
    False,
    Quote(Box<Expr>),
    List(LinkedList<Expr>),
}
