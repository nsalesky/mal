use std::collections::LinkedList;
use std::iter::Peekable;
use std::num::ParseIntError;
use std::str::Chars;

use thiserror::Error;

use crate::parser::ParseError::EmptyExpr;
use crate::types::Expr;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("expected an expression but got an empty string")]
    EmptyExpr,

    #[error("invalid integer")]
    IntegerParseError(#[from] ParseIntError),

    #[error("parentheses were unbalanced")]
    UnbalancedParens,

    #[error("integer contained a non-numeric character")]
    IntegerContainsNonNumericChar,

    #[error("the input string was invalid: {0}")]
    InvalidExpr(String),
}

pub fn parse_string(text: &str) -> Result<Expr, ParseError> {
    let mut chars = text.chars().peekable();
    parse_chars(&mut chars)
}

fn parse_chars(chars: &mut Peekable<Chars>) -> Result<Expr, ParseError> {
    loop {
        match chars.peek() {
            Some(c) => match c {
                '(' => return parse_list(chars),
                c if c.is_digit(10) => return parse_integer(chars),
                ' ' => { chars.next(); }
                ')' => return Err(ParseError::UnbalancedParens),
                _ => return parse_symbol(chars)
            },
            None => return Err(EmptyExpr)
        }
    }
}

fn parse_list(chars: &mut Peekable<Chars>) -> Result<Expr, ParseError> {
    let mut inner_text = String::new();
    let mut bracket_stack = LinkedList::new();

    match chars.next() {
        Some('(') => bracket_stack.push_front('('),
        _ => return Err(ParseError::InvalidExpr("string did not begin with an opening paren".to_string()))
    }

    while let Some(c) = chars.next() {
        match c {
            ')' => match bracket_stack.pop_front() {
                Some('(') => {
                    if bracket_stack.is_empty() {
                        break;
                    } else {
                        inner_text.push(c);
                    }
                }
                _ => return Err(ParseError::UnbalancedParens)
            }
            '(' => {
                bracket_stack.push_front(c);
                inner_text.push(c);
            }
            _ => { inner_text.push(c) }
        }
    }

    let mut expr_elements = LinkedList::new();

    let mut inner_text_chars = inner_text.chars().peekable();
    while inner_text_chars.peek().is_some() {
        match parse_chars(&mut inner_text_chars) {
            Ok(expr) => expr_elements.push_back(expr),
            Err(ParseError::EmptyExpr) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(Expr::List(expr_elements))
}

fn parse_symbol(chars: &mut Peekable<Chars>) -> Result<Expr, ParseError> {
    let mut symbol_str = String::new();

    while let Some(c) = chars.next() {
        match c {
            _ if c.is_whitespace() => break,
            c => { symbol_str.push(c) }
        }
    }

    Ok(Expr::Symbol(symbol_str))
}

fn parse_integer(chars: &mut Peekable<Chars>) -> Result<Expr, ParseError> {
    let mut integer_str = String::new();

    while let Some(c) = chars.next() {
        match c {
            c if c.is_digit(10) => { integer_str.push(c); }
            c if c.is_whitespace() => break,
            _ => return Err(ParseError::IntegerContainsNonNumericChar),
        }
    }

    match integer_str.parse::<i64>() {
        Ok(val) => Ok(Expr::Integer(val)),
        Err(e) => Err(ParseError::IntegerParseError(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_string() {
        assert_eq!(Err(EmptyExpr), parse_string(""));
    }

    #[test]
    fn test_parse_symbol() {
        assert_eq!(Ok(Expr::Symbol("foo".to_string())), parse_string("foo"));
        assert_eq!(Ok(Expr::Symbol("foo-bar".to_string())), parse_string(" foo-bar   "));
        assert_eq!(Ok(Expr::Symbol("hi".to_string())), parse_string("    hi"));
    }

    #[test]
    fn test_parse_integer() {
        assert_eq!(Ok(Expr::Integer(63)), parse_string("63"));
        assert_eq!(Ok(Expr::Integer(18)), parse_string(" 18"));
        assert_eq!(Ok(Expr::Integer(100)), parse_string(" 100    "));
        assert_eq!(Ok(Expr::Integer(3)), parse_string("      3    "));
    }

    #[test]
    fn test_parse_integer_invalid() {
        assert_eq!(Err(ParseError::IntegerContainsNonNumericChar), parse_string("1a"));
        assert_eq!(Err(ParseError::IntegerContainsNonNumericChar), parse_string("12;"));
    }

    #[test]
    fn test_parse_list() {
        let expected_expr = Expr::List(LinkedList::from([
            Expr::Symbol("add".to_string()),
            Expr::Integer(32),
            Expr::Integer(4)
        ]));
        assert_eq!(Ok(expected_expr), parse_string(" (add   32      4) "));
    }

    #[test]
    fn test_parse_list_unbalance() {
        assert_eq!(Err(ParseError::UnbalancedParens), parse_string("(+ 1 2))"));
    }

    #[test]
    fn test_parse_nested_list() {
        let expected_expr = Expr::List(LinkedList::from([
            Expr::Symbol("+".to_string()),
            Expr::Integer(2),
            Expr::List(LinkedList::from([
                Expr::Symbol("*".to_string()),
                Expr::Integer(3),
                Expr::Integer(4)
            ]))
        ]));
        assert_eq!(Ok(expected_expr), parse_string("(  + 2   (*  3  4)  )  "))
    }
}
