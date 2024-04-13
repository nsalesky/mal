use thiserror::Error;

use crate::parser::ParseError;

pub mod types;
pub mod parser;
pub mod printer;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("parse error: {0}")]
    ParseError(#[from] ParseError)
}
