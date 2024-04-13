use std::fmt::Write;

use thiserror::Error;

use crate::parser::{parse_text_to_expressions, ParseError};

pub mod types;
pub mod parser;
pub mod printer;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("parse error: {0}")]
    ParseError(#[from] ParseError)
}

pub fn rep(input: &str) -> Result<String, RuntimeError> {
    let exprs = parse_text_to_expressions(input)?;

    let mut output = String::new();
    for expr in exprs {
        writeln!(output, "{}", expr).expect("to be able to write to a string");
    }

    Ok(output)
}
