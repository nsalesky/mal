extern crate core;

use std::fmt::Write;

pub use env::Environment;

use crate::evaluator::{evaluate_expr, RuntimeError};
use crate::parser::parse_text_to_expressions;

mod types;
mod parser;
mod printer;
mod evaluator;
mod env;
mod builtins;

pub fn rep(input: &str, env: &mut Environment) -> Result<String, RuntimeError> {
    let exprs = parse_text_to_expressions(input)?;

    let mut output = String::new();
    for expr in exprs {
        let result = evaluate_expr(expr, env)?;
        writeln!(output, "{}", result).expect("to be able to write to a string");
    }

    Ok(output)
}
