extern crate core;

use std::fmt::Write;

pub use env::Env;

use crate::evaluator::{evaluate_expr, RuntimeError};
use crate::parser::parse_text_to_expressions;

mod types;
mod parser;
mod printer;
mod evaluator;
mod env;
mod builtins;

pub type Result<T> = std::result::Result<T, RuntimeError>;

pub fn rep(input: &str, env: &Env) -> Result<String> {
    let exprs = parse_text_to_expressions(input)?;

    let mut output = String::new();
    for expr in exprs {
        let result = evaluate_expr(expr, env)?;
        writeln!(output, "{}", result).expect("to be able to write to a string");
    }

    Ok(output)
}
