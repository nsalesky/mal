use std::fmt::{Display, Formatter};

use crate::types::Expr;

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Expr::Integer(val) => {
                write!(f, "{}", val)
            }
            Expr::Symbol(val) => {
                write!(f, "{}", val)
            }
            Expr::List(elements) => {
                write!(f, "(")?;
                for (i, elem) in elements.into_iter().enumerate() {
                    write!(f, "{}", elem)?;
                    if i < elements.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use crate::types::Expr;

    #[test]
    fn test_display_integer() {
        assert_eq!("12", Expr::Integer(12).to_string());
        assert_eq!("5", Expr::Integer(5).to_string());
    }

    #[test]
    fn test_display_symbol() {
        assert_eq!("+", Expr::Symbol("+".to_string()).to_string());
        assert_eq!("foo-bar", Expr::Symbol("foo-bar".to_string()).to_string());
    }

    #[test]
    fn test_display_list() {
        assert_eq!("(1)", Expr::List(LinkedList::from([Expr::Integer(1)])).to_string());

        assert_eq!("(+ 1 2)", Expr::List(LinkedList::from([
            Expr::Symbol("+".to_string()),
            Expr::Integer(1),
            Expr::Integer(2)
        ])).to_string());
    }

    #[test]
    fn test_display_nested_list() {
        assert_eq!("(+ (* 12 8) 2)", Expr::List(LinkedList::from([
            Expr::Symbol("+".to_string()),
            Expr::List(LinkedList::from([
                Expr::Symbol("*".to_string()),
                Expr::Integer(12),
                Expr::Integer(8),
            ])),
            Expr::Integer(2)
        ])).to_string());
    }
}