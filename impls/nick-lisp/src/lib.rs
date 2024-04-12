use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {

}

#[derive(Error, Debug)]
pub enum RuntimeError {

}

pub fn rep(input: &str) -> Result<String, RuntimeError> {
    Ok("hello".to_string())
}