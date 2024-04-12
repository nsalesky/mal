use thiserror::Error;

mod printer;
mod reader;

#[derive(Error, Debug)]
pub enum ParseError {

}

#[derive(Error, Debug)]
pub enum RuntimeError {

}

pub fn rep(input: &str) -> Result<String, RuntimeError> {
    Ok(input.to_string())
}