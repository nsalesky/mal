use nlisp::{Environment, rep, Result};

#[test]
fn test_parse_and_print_strings() -> Result<()> {
    let input = "\"\\\"\""; // "\""
    let output = rep(input, &mut Environment::default())?;
    assert_eq!("\"\\\"\"\n", output);
    Ok(())
}