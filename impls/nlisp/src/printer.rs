use std::fmt::{Display, Formatter};

use crate::types::Expr;

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Expr::Integer(val) => {
                write!(f, "{}", val)
            }
            Expr::Symbol(val) | Expr::Keyword(val) => {
                write!(f, "{}", val)
            }
            Expr::String(val) => {
                write!(f, "\"{}\"", val)
            }
            Expr::Nil => {
                write!(f, "nil")
            }
            Expr::True => {
                write!(f, "true")
            }
            Expr::False => {
                write!(f, "false")
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
            Expr::Vector(elements) => {
                write!(f, "[")?;
                for (i, elem) in elements.into_iter().enumerate() {
                    write!(f, "{}", elem)?;
                    if i < elements.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            }
            Expr::HashMap(pairs) => {
                write!(f, "{{")?;
                for (i, (key, value)) in pairs.iter().enumerate() {
                    write!(f, "{} {}", key, value)?;
                    if i < pairs.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "}}")
            }
            Expr::Quote(expr) => {
                write!(f, "(quote {})", expr)
            }
            Expr::Quasiquote(expr) => {
                write!(f, "(quasiquote {})", expr)
            }
            Expr::Unquote(expr) => {
                write!(f, "(unquote {})", expr)
            }
            Expr::SpliceUnquote(expr) => {
                write!(f, "(splice-unquote {})", expr)
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
    fn test_display_keyword() {
        assert_eq!(":foo", Expr::Keyword(":foo".to_string()).to_string());
        assert_eq!(":foo-bar", Expr::Keyword(":foo-bar".to_string()).to_string());
    }

    #[test]
    fn test_display_string() {
        assert_eq!("\"hello world\"", Expr::String("hello world".to_string()).to_string());
        assert_eq!("\"foo\nbar\"", Expr::String("foo\nbar".to_string()).to_string());
    }

    #[test]
    fn test_display_identifiers() {
        assert_eq!("nil", Expr::Nil.to_string());
        assert_eq!("true", Expr::True.to_string());
        assert_eq!("false", Expr::False.to_string());
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

    #[test]
    fn test_display_vector() {
        assert_eq!("[1]", Expr::Vector(vec![Expr::Integer(1)]).to_string());

        assert_eq!("[foo 1 2]", Expr::Vector(vec![
            Expr::Symbol("foo".to_string()),
            Expr::Integer(1),
            Expr::Integer(2),
        ]).to_string());
    }

    #[test]
    fn test_display_hashmap() {
        assert_eq!("{\"foo\" 1}", Expr::HashMap(vec![(Expr::String("foo".to_string()), Expr::Integer(1))]).to_string());

        let hashmap_expr = Expr::HashMap(vec![
            (Expr::Keyword(":foo".to_string()), Expr::String("bar".to_string())),
            (Expr::Keyword(":baz".to_string()), Expr::Integer(3)),
        ]);
        assert_eq!("{:foo \"bar\" :baz 3}", hashmap_expr.to_string());
    }

    #[test]
    fn test_display_quote() {
        assert_eq!("(quote a)", Expr::Quote(Box::new(Expr::Symbol("a".to_string()))).to_string());
        assert_eq!("(quote 123)", Expr::Quote(Box::new(Expr::Integer(123))).to_string());

        let list_expr = Expr::List(LinkedList::from([
            Expr::Symbol("a".to_string()),
            Expr::Integer(1),
            Expr::True
        ]));
        assert_eq!("(quote (a 1 true))", Expr::Quote(Box::new(list_expr)).to_string());
    }

    #[test]
    fn test_display_quasiquote() {
        assert_eq!("(quasiquote a)", Expr::Quasiquote(Box::new(Expr::Symbol("a".to_string()))).to_string());
        assert_eq!("(quasiquote 123)", Expr::Quasiquote(Box::new(Expr::Integer(123))).to_string());
    }
}