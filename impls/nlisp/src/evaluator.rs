use thiserror::Error;

use crate::env::Environment;
use crate::parser::ParseError;
use crate::types::Expr;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("parse error: {0}")]
    ParseError(#[from] ParseError)
}

pub fn evaluate_expr(expr: Expr, env: &mut Environment) -> Result<Expr, RuntimeError> {
    Ok(expr)
}