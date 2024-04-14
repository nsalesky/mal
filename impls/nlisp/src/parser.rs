use std::collections::LinkedList;
use std::iter::Peekable;
use std::num::ParseIntError;
use std::str::Chars;

use itertools::Itertools;
use thiserror::Error;

use crate::parser::ParseError::EmptyExpr;
use crate::types::Expr;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("expected an expression but got an empty string")]
    EmptyExpr,

    #[error("invalid integer")]
    IntegerParseError(#[from] ParseIntError),

    #[error("parentheses were unbalanced in expression")]
    UnbalancedParens,

    #[error("integer contained a non-numeric character: `{0}`")]
    IntegerContainsNonNumericChar(char),

    #[error("string was invalid")]
    InvalidString,

    #[error("invalid backslash character in string")]
    StringInvalidBackslash,

    #[error("hashmap is missing a value for key `{0}`")]
    HashmapMissingValue(String),

    #[error("the input string was invalid: {0}")]
    InvalidExpr(String),
}

pub fn parse_text_to_expression(text: &str) -> Result<Expr, ParseError> {
    let mut chars = text.chars().peekable();
    parse_chars(&mut chars)
}

pub fn parse_text_to_expressions(text: &str) -> Result<Vec<Expr>, ParseError> {
    let mut chars = text.chars().peekable();
    let mut exprs = vec![];

    while chars.peek().is_some() {
        match parse_chars(&mut chars) {
            Ok(expr) => exprs.push(expr),
            Err(ParseError::EmptyExpr) => break,
            Err(e) => return Err(e)
        }
    }

    Ok(exprs)
}

pub fn parse_chars(chars: &mut Peekable<Chars>) -> Result<Expr, ParseError> {
    loop {
        match chars.peek() {
            Some(c) => match c {
                c if c.is_digit(10) => return parse_integer(chars),
                c if c.is_whitespace() => { chars.next(); }
                ',' => { chars.next(); }
                ';' => consume_comment(chars)?,
                '(' => return parse_list(chars),
                ')' => return Err(ParseError::UnbalancedParens),
                '[' => return parse_vector(chars),
                ']' => return Err(ParseError::UnbalancedParens),
                '{' => return parse_hashmap(chars),
                '}' => return Err(ParseError::UnbalancedParens),
                '\"' => return parse_string(chars),
                '\'' => {
                    chars.next();
                    return match parse_chars(chars) {
                        Ok(expr) => Ok(Expr::Quote(Box::new(expr))),
                        Err(e) => Err(e)
                    };
                }
                ':' => return match parse_symbol(chars) {
                    Ok(Expr::Symbol(val)) => Ok(Expr::Keyword(val)),
                    Ok(_) => Err(ParseError::InvalidExpr("keyword was not parsed as a symbol".to_string())),
                    Err(e) => Err(e),
                },
                _ => return parse_symbol(chars)
            },
            None => return Err(EmptyExpr)
        }
    }
}

fn consume_comment(chars: &mut Peekable<Chars>) -> Result<(), ParseError> {
    while let Some(c) = chars.peek() {
        if *c == '\n' {
            chars.next();
            break;
        } else {
            chars.next();
        }
    }
    Ok(())
}

fn consume_chars_between(chars: &mut Peekable<Chars>, open_delim: char, close_delim: char) -> Result<String, ParseError> {
    let mut inner_text = String::new();
    let mut num_opening_delimeters: u32 = 0;

    match chars.next() {
        Some(c) if c == open_delim => num_opening_delimeters += 1,
        _ => return Err(ParseError::InvalidExpr("expression did not begin with an opening delimiter".to_string()))
    }

    while let Some(c) = chars.next() {
        match c {
            c if c == open_delim => {
                num_opening_delimeters += 1;
                inner_text.push(c);
            }
            c if c == close_delim => {
                if num_opening_delimeters <= 0 {
                    return Err(ParseError::UnbalancedParens);
                } else if num_opening_delimeters == 1 {
                    num_opening_delimeters -= 1;
                    break;
                } else {
                    num_opening_delimeters -= 1;
                    inner_text.push(c);
                }
            }
            ';' => {
                consume_comment(chars)?;
                inner_text.push(' ');
            }
            _ => inner_text.push(c)
        }
    }

    if num_opening_delimeters > 0 {
        return Err(ParseError::UnbalancedParens);
    }

    Ok(inner_text)
}

fn parse_list(chars: &mut Peekable<Chars>) -> Result<Expr, ParseError> {
    let inner_text = consume_chars_between(chars, '(', ')')?;

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

fn parse_vector(chars: &mut Peekable<Chars>) -> Result<Expr, ParseError> {
    let inner_text = consume_chars_between(chars, '[', ']')?;

    let mut expr_elements = vec![];

    let mut inner_text_chars = inner_text.chars().peekable();
    while inner_text_chars.peek().is_some() {
        match parse_chars(&mut inner_text_chars) {
            Ok(expr) => expr_elements.push(expr),
            Err(ParseError::EmptyExpr) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(Expr::Vector(expr_elements))
}

fn parse_hashmap(chars: &mut Peekable<Chars>) -> Result<Expr, ParseError> {
    let inner_text = consume_chars_between(chars, '{', '}')?;
    let inner_expressions = parse_text_to_expressions(inner_text.as_str())?;

    if inner_expressions.len() % 2 != 0 {
        return Err(ParseError::HashmapMissingValue(inner_expressions[inner_expressions.len() - 1].to_string()));
    }


    let map_pairs = inner_expressions
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut pair| {
            let key = pair.next().expect("hashmap pair to have a key");
            let value = pair.next().expect("hashmap pair to have a value");

            (key.to_owned(), value.to_owned())
        })
        .collect();

    Ok(Expr::HashMap(map_pairs))
}

fn parse_symbol(chars: &mut Peekable<Chars>) -> Result<Expr, ParseError> {
    let mut symbol_str = String::new();

    while let Some(c) = chars.next() {
        match c {
            ';' => consume_comment(chars)?,
            c if c.is_whitespace() => break,
            ',' => break,
            c => { symbol_str.push(c) }
        }
    }

    // check if this corresponds to a special identifier
    match symbol_str.as_str() {
        "nil" => Ok(Expr::Nil),
        "true" | "#t" => Ok(Expr::True),
        "false" | "#f" => Ok(Expr::False),
        _ => Ok(Expr::Symbol(symbol_str))
    }
}

fn parse_integer(chars: &mut Peekable<Chars>) -> Result<Expr, ParseError> {
    let mut integer_str = String::new();

    while let Some(c) = chars.peek() {
        match c {
            c if c.is_digit(10) => {
                integer_str.push(*c);
                chars.next();
            }
            c if c.is_whitespace() => {
                chars.next();
                break;
            }
            ';' => break,
            ',' => break,
            _ => return Err(ParseError::IntegerContainsNonNumericChar(*c)),
        }
    }

    match integer_str.parse::<i64>() {
        Ok(val) => Ok(Expr::Integer(val)),
        Err(e) => Err(ParseError::IntegerParseError(e))
    }
}

fn parse_string(chars: &mut Peekable<Chars>) -> Result<Expr, ParseError> {
    // 1. make sure the string begins with a double-quote
    match chars.next() {
        Some('"') => {}
        _ => return Err(ParseError::InvalidString)
    }

    let mut string_contents = String::new();

    while let Some(c) = chars.next() {
        match c {
            '"' => break,
            '/' => match chars.next() {
                Some('"') => string_contents.push('"'),
                Some('n') => string_contents.push('\n'),
                Some('\\') => string_contents.push('\\'),
                _ => return Err(ParseError::StringInvalidBackslash),
            }
            _ => string_contents.push(c),
        }
    }

    Ok(Expr::String(string_contents))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_string() {
        assert_eq!(Err(ParseError::EmptyExpr), parse_text_to_expression(""));
    }

    #[test]
    fn test_parse_symbol() {
        assert_eq!(Ok(Expr::Symbol("foo".to_string())), parse_text_to_expression("foo"));
        assert_eq!(Ok(Expr::Symbol("foo-bar".to_string())), parse_text_to_expression(" foo-bar   "));
        assert_eq!(Ok(Expr::Symbol("hi".to_string())), parse_text_to_expression("    hi"));
    }

    #[test]
    fn test_parse_integer() {
        assert_eq!(Ok(Expr::Integer(63)), parse_text_to_expression("63"));
        assert_eq!(Ok(Expr::Integer(18)), parse_text_to_expression(" 18"));
        assert_eq!(Ok(Expr::Integer(100)), parse_text_to_expression(" 100    "));
        assert_eq!(Ok(Expr::Integer(3)), parse_text_to_expression("      3    "));
    }

    #[test]
    fn test_parse_integer_invalid() {
        assert_eq!(Err(ParseError::IntegerContainsNonNumericChar('a')), parse_text_to_expression("1a"));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(Ok(Expr::String("hello world".to_string())), parse_text_to_expression("\"hello world\""));
        assert_eq!(Ok(Expr::String("hello\nworld".to_string())), parse_text_to_expression("\"hello\nworld\""));
    }

    #[test]
    fn test_parse_list() {
        let expected_expr = Expr::List(LinkedList::from([
            Expr::Symbol("add".to_string()),
            Expr::Integer(32),
            Expr::Integer(4)
        ]));
        assert_eq!(Ok(expected_expr), parse_text_to_expression(" (add   32      4) "));
    }

    #[test]
    fn test_parse_list_unbalance() {
        assert_eq!(Err(ParseError::UnbalancedParens),
                   parse_text_to_expression("(+ 1 2"));
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
        assert_eq!(Ok(expected_expr), parse_text_to_expression("(  + 2   (*  3  4)  )  "))
    }

    #[test]
    fn test_parse_vector() {
        let expected_expr = Expr::Vector(vec![
            Expr::Symbol("foo".to_string()),
            Expr::Integer(32),
            Expr::String("hi there".to_string()),
        ]);
        assert_eq!(Ok(expected_expr), parse_text_to_expression(" [ foo   32      \"hi there\" ] "));
    }

    #[test]
    fn test_parse_hashmap() {
        assert_eq!(Ok(Expr::HashMap(vec![])), parse_text_to_expression("{}"));

        let expected_expr = Expr::HashMap(vec![
            (Expr::Keyword(":foo".to_string()), Expr::Integer(32)),
            (Expr::Keyword(":bar".to_string()), Expr::String("hi there".to_string())),
        ]);
        assert_eq!(Ok(expected_expr), parse_text_to_expression(" { :foo  32 :bar     \"hi there\" } "));
    }

    #[test]
    fn test_parse_hashmap_unbalanced() {
        assert_eq!(Err(ParseError::HashmapMissingValue(":bar".to_string())),
                   parse_text_to_expression("{:bar} "));
    }


    #[test]
    fn test_parse_comment() {
        assert_eq!(Ok(Expr::Integer(8)), parse_text_to_expression(" 8;      \n"));

        let expected_expr = Expr::List(LinkedList::from([
            Expr::Symbol("-".to_owned()),
            Expr::Integer(3),
            Expr::Integer(1)
        ]));
        assert_eq!(Ok(expected_expr), parse_text_to_expression("(- 3;\n1)"));
    }

    #[test]
    fn test_parse_identifiers() {
        assert_eq!(Ok(Expr::Nil), parse_text_to_expression("nil"));
        assert_eq!(Ok(Expr::True), parse_text_to_expression("true"));
        assert_eq!(Ok(Expr::True), parse_text_to_expression("#t"));
        assert_eq!(Ok(Expr::False), parse_text_to_expression("false"));
        assert_eq!(Ok(Expr::False), parse_text_to_expression("#f"));
    }

    #[test]
    fn test_parse_quote() {
        assert_eq!(Ok(Expr::Quote(Box::new(Expr::Symbol("a".to_string())))),
                   parse_text_to_expression("'a"));
    }

    #[test]
    fn test_parse_newlines() {
        assert_eq!(Ok(Expr::Integer(1)), parse_text_to_expression("\n1\n\n"));
    }
}
