use nlisp::{Env, rep, Result};

#[test]
fn test_parse_and_print_strings() -> Result<()> {
    let input = "\"\\\"\""; // "\""
    let output = rep(input, &Env::default())?;
    assert_eq!("\"\\\"\"\n", output);
    Ok(())
}

#[test]
fn test_recursive_functions() -> Result<()> {
    let function_def_src = "(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))";
    let env = Env::default();
    assert_eq!("(fn ...)\n", rep(function_def_src, &env)?);
    assert_eq!("1\n", rep("(fib 0)", &env)?);
    assert_eq!("1\n", rep("(fib 1)", &env)?);
    assert_eq!("2\n", rep("(fib 2)", &env)?);
    Ok(())
}