use std::collections::LinkedList;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Integer(i64),
    String(String),
    Symbol(String),
    Keyword(String),
    Nil,
    True,
    False,
    Quote(Box<Expr>),
    List(LinkedList<Expr>),
    Vector(Vec<Expr>),
    HashMap(Vec<(Expr, Expr)>),
}
