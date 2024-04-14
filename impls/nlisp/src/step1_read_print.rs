use std::io;
use std::io::Write;

use nlisp::rep;

fn main() -> Result<(), io::Error> {
    loop {
        print!("user> ");
        io::stdout().flush()?;

        let mut input_buffer = String::new();
        let bytes = io::stdin().read_line(&mut input_buffer)?;

        if bytes == 0 {
            return Ok(());
        }

        match rep(input_buffer.as_str()) {
            Ok(output) => print!("{}", output),
            Err(e) => println!("error: {}", e),
        }
    }
}
