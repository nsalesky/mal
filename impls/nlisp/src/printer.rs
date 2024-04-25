use std::fmt::{Display, Formatter};

use crate::types::{Expr, HashableValue, Value};

pub fn write_delimiter_separated_elems<T>(elems: T, delimiter: &str) -> String
    where T: IntoIterator,
          T::Item: Display
{
    let mut result = String::new();
    for (i, elem) in elems.into_iter().enumerate() {
        if i > 0 {
            result.push_str(delimiter);
        }
        result.push_str(&format!("{}", elem));
    }
    result
}

pub fn write_delimiter_separated_printables<T>(elems: T, delimiter: &str, print_readably: bool) -> String
    where T: IntoIterator,
          T::Item: Printable
{
    let mut result = String::new();

    for (i, elem) in elems.into_iter().enumerate() {
        if i > 0 {
            result.push_str(delimiter);
        }
        result.push_str(&elem.print_value(print_readably));
    }
    result
}

pub trait Printable {
    fn print_value(&self, readable: bool) -> String;
}

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
            Expr::Boolean(b) => {
                if *b {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Expr::List(elements) => {
                write!(f, "({})", write_delimiter_separated_elems(elements, " "))
            }
            Expr::Vector(elements) => {
                write!(f, "[{}]", write_delimiter_separated_elems(elements, " "))
            }
            Expr::HashMap(pairs) => {
                let inner_str = write_delimiter_separated_elems(pairs.iter().map(|(key, value)|
                    format!("{} {}", key, value)
                ), " ");
                write!(f, "{{{}}}", inner_str)
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

impl Printable for Value {
    fn print_value(&self, readable: bool) -> String {
        match self {
            Value::Integer(val) => {
                format!("{}", val)
            }
            Value::Symbol(val) | Value::Keyword(val) => {
                format!("{}", val)
            }
            Value::String(val) => {
                if readable {
                    let mut result = String::new();

                    for c in val.chars() {
                        match c {
                            '\"' => result.push_str("\\\""),
                            '\\' => result.push_str("\\\\"),
                            '\n' => result.push_str("\\n"),
                            _ => result.push(c),
                        }
                    }
                    format!("\"{}\"", result)
                } else {
                    val.to_string()
                }
            }
            Value::Nil => {
                "nil".to_string()
            }
            Value::Boolean(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Value::List(elements) => {
                format!("({})", write_delimiter_separated_printables(elements.iter().cloned(), " ", readable))
            }
            Value::Vector(elements) => {
                format!("[{}]", write_delimiter_separated_printables(elements.iter().cloned(), " ", readable))
            }
            Value::HashMap(pairs) => {
                let elements = pairs.iter().map(|(key, value)|
                    format!("{} {}", key.print_value(readable), value.print_value(readable))
                );

                format!("{{{}}}", write_delimiter_separated_elems(elements, " "))
            }
            Value::Function(_function_body) => {
                "(fn ...)".to_string()
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print_value(true))
    }
}

impl Display for HashableValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HashableValue::Integer(num) => write!(f, "{}", num),
            HashableValue::String(s) => write!(f, "\"{}\"", s),
            HashableValue::Keyword(s) => write!(f, "{}", s),
        }
    }
}

impl Printable for HashableValue {
    fn print_value(&self, readable: bool) -> String {
        match self {
            HashableValue::Integer(num) => format!("{}", num),
            HashableValue::String(s) => Value::String(s.to_string()).print_value(readable),
            HashableValue::Keyword(s) => s.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_integer() {
        assert_eq!("12", Value::Integer(12).to_string());
        assert_eq!("5", Value::Integer(5).to_string());
    }

    #[test]
    fn test_display_symbol() {
        assert_eq!("+", Value::Symbol("+".to_string()).to_string());
        assert_eq!("foo-bar", Value::Symbol("foo-bar".to_string()).to_string());
    }

    #[test]
    fn test_display_keyword() {
        assert_eq!(":foo", Value::Keyword(":foo".to_string()).to_string());
        assert_eq!(":foo-bar", Value::Keyword(":foo-bar".to_string()).to_string());
    }

    #[test]
    fn test_display_string() {
        assert_eq!("\"hello world\"", Value::String("hello world".to_string()).to_string());
        assert_eq!("\"foo\\nbar\"", Value::String("foo\nbar".to_string()).to_string());
    }

    #[test]
    fn test_print_string_raw() {
        assert_eq!("hello\nworld", Value::String("hello\nworld".to_string()).print_value(false));
    }

    #[test]
    fn test_display_identifiers() {
        assert_eq!("nil", Value::Nil.to_string());
        assert_eq!("true", Value::Boolean(true).to_string());
        assert_eq!("false", Value::Boolean(false).to_string());
    }

    #[test]
    fn test_display_list() {
        assert_eq!("(1)", Value::List(rpds::List::from_iter([Value::Integer(1)])).to_string());

        assert_eq!("(+ 1 2)", Value::List(rpds::List::from_iter([
            Value::Symbol("+".to_string()),
            Value::Integer(1),
            Value::Integer(2)
        ])).to_string());
    }

    #[test]
    fn test_display_nested_list() {
        assert_eq!("(+ (* 12 8) 2)", Value::List(rpds::List::from_iter([
            Value::Symbol("+".to_string()),
            Value::List(rpds::List::from_iter([
                Value::Symbol("*".to_string()),
                Value::Integer(12),
                Value::Integer(8),
            ])),
            Value::Integer(2)
        ])).to_string());
    }

    #[test]
    fn test_display_vector() {
        assert_eq!("[1]", Value::Vector(rpds::Vector::from_iter([Value::Integer(1)])).to_string());

        assert_eq!("[foo 1 2]", Value::Vector(rpds::Vector::from_iter([
            Value::Symbol("foo".to_string()),
            Value::Integer(1),
            Value::Integer(2),
        ])).to_string());
    }

    #[test]
    fn test_display_hashmap() {
        assert_eq!("{\"foo\" 1}", Value::HashMap(
            rpds::HashTrieMap::from_iter([(HashableValue::String("foo".to_string()), Value::Integer(1))])).to_string());
    }

    // #[test]
    // fn test_display_quote() {
    //     assert_eq!("(quote a)", Expr::Quote(Box::new(Expr::Symbol("a".to_string()))).to_string());
    //     assert_eq!("(quote 123)", Expr::Quote(Box::new(Expr::Integer(123))).to_string());
    //
    //     let list_expr = Expr::List(LinkedList::from([
    //         Expr::Symbol("a".to_string()),
    //         Expr::Integer(1),
    //         Expr::Boolean(true)
    //     ]));
    //     assert_eq!("(quote (a 1 true))", Expr::Quote(Box::new(list_expr)).to_string());
    // }
    //
    // #[test]
    // fn test_display_quasiquote() {
    //     assert_eq!("(quasiquote a)", Expr::Quasiquote(Box::new(Expr::Symbol("a".to_string()))).to_string());
    //     assert_eq!("(quasiquote 123)", Expr::Quasiquote(Box::new(Expr::Integer(123))).to_string());
    // }

    // #[test]
    // fn test_display_function() {
    //     let val = Value::Function(FunctionBody::BuiltinValues(builtins::add));
    //
    //     assert_eq!("(fn ...)", val.to_string());
    // }
}