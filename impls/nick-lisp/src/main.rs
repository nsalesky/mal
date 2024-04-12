use std::io;
use std::io::Write;
use nick_lisp::rep;

fn main() -> Result<(), io::Error> {
    loop {
        print!("user> ");
        io::stdout().flush()?;

        let mut input_buffer = String::new();
        io::stdin().read_line(&mut input_buffer)?;

        if input_buffer.is_empty() {
            break;
        }

        match rep(&input_buffer) {
            Ok(result_value) => println!("{}", result_value),
            Err(e) => println!("Runtime error: {}", e)
        }
    }
    Ok(())
}
