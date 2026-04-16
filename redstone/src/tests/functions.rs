use crate::tests::try_run;

#[test]
fn test_function_call() -> Result<(), String> {
    const SRC: &str = r#"
    fn add(a: i64, b: i64) -> i64 {
        return a + b;
    }

    fn main() {
        let x = add(3, 4);
        print(x);
        return 0;
    }
    "#;

    let result = try_run(SRC)?.output().map_err(|e| format!("program run failed: {e}"))?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr)
            .map_err(|e| format!("read utf8 from stderr: {e}"))?);
    }

    assert_eq!(
        String::from_utf8(result.stdout).map_err(|e| format!("read utf8 from stdout: {e}"))?,
        "7\n"
    );

    Ok(())
}

#[test]
fn test_expr_in_args() -> Result<(), String> {
    const SRC: &str = r#"
    fn add(a: i64, b: i64) -> i64 {
        return a + b;
    }

    fn main() {
        let x = add(2 * 3, 10 - 4);
        print(x);
        return 0;
    }
    "#;

    let result = try_run(SRC)?.output().map_err(|e| format!("program run failed: {e}"))?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr)
            .map_err(|e| format!("read utf8 from stderr: {e}"))?);
    }

    assert_eq!(
        String::from_utf8(result.stdout).map_err(|e| format!("read utf8 from stdout: {e}"))?,
        "12\n"
    );

    Ok(())
}

#[test]
fn test_nested_calls() -> Result<(), String> {
    const SRC: &str = r#"
    fn inc(x: i64) -> i64 {
        return x + 1;
    }

    fn double(x: i64) -> i64 {
        return x * 2;
    }

    fn main() {
        print(double(inc(5)));
        return 0;
    }
    "#;

    let result = try_run(SRC)?.output().map_err(|e| format!("program run failed: {e}"))?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr)
            .map_err(|e| format!("read utf8 from stderr: {e}"))?);
    }

    assert_eq!(
        String::from_utf8(result.stdout).map_err(|e| format!("read utf8 from stdout: {e}"))?,
        "12\n"
    );

    Ok(())
}

#[test]
fn test_void_function() -> Result<(), String> {
    const SRC: &str = r#"
    fn greet() {
        print(72);
    }

    fn main() {
        greet();
        return 0;
    }
    "#;

    let result = try_run(SRC)?.output().map_err(|e| format!("program run failed: {e}"))?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr)
            .map_err(|e| format!("read utf8 from stderr: {e}"))?);
    }

    assert_eq!(
        String::from_utf8(result.stdout).map_err(|e| format!("read utf8 from stdout: {e}"))?,
        "72\n"
    );

    Ok(())
}
