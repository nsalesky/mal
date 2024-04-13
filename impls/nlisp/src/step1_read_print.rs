use std::io;
use std::io::Write;

use nlisp::parser::{parse_chars, ParseError};

fn main() -> Result<(), io::Error> {
    let mut previous_unfinished_input = "".to_string();

    let is_interactive = false;

    loop {
        if previous_unfinished_input.is_empty() {
            print!("user> ");
            io::stdout().flush()?;
        }

        let mut input_buffer = String::new();
        let bytes = io::stdin().read_line(&mut input_buffer)?;

        if bytes == 0 {
            return Ok(());
        }

        let mut current_input = previous_unfinished_input.clone();
        current_input.push_str(input_buffer.trim());

        let mut chars = current_input.chars().peekable();

        while chars.peek().is_some() {
            match parse_chars(&mut chars) {
                Ok(expr) => {
                    println!("{}", expr.to_string());
                    previous_unfinished_input.clear();
                }
                Err(ParseError::EmptyExpr) => { previous_unfinished_input.clear() }
                Err(ParseError::UnbalancedParens(unfinished_input)) => {
                    if is_interactive {
                        previous_unfinished_input = unfinished_input;
                    } else {
                        println!("parser error: unbalanced parens");
                    }
                }
                Err(e) => {
                    previous_unfinished_input.clear();
                    println!("parser error: {}", e);
                }
            }
        }
    }
}
