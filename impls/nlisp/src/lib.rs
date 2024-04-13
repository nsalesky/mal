use thiserror::Error;

use crate::parser::{parse_string, ParseError};

mod types;
mod parser;
mod printer;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("parse error: {0}")]
    ParseError(#[from] ParseError)
}

pub fn rep(input: &str) -> Result<String, RuntimeError> {
    let expr = parse_string(input)?;

    Ok(expr.to_string())
}