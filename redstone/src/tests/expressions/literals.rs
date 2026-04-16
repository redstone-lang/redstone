use crate::tests::try_run;

#[test]
fn test_integer_literal() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        print(42);
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
        "42\n"
    );

    Ok(())
}

#[test]
fn test_float_literal() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        let pi: f64 = 3.14;
        print(pi);
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
        "3.14\n"
    );

    Ok(())
}

#[test]
fn test_bool_literal() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        print(true);
        print(false);
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
        "1\n0\n"
    );

    Ok(())
}

#[test]
fn test_char_literal() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        print('A');
        return 0;
    }
    "#;

    let result = try_run(SRC)?.output().map_err(|e| format!("program run failed: {e}"))?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr)
            .map_err(|e| format!("read utf8 from stderr: {e}"))?);
    }

    // 'A' is printed as its numeric code point: 65
    assert_eq!(
        String::from_utf8(result.stdout).map_err(|e| format!("read utf8 from stdout: {e}"))?,
        "65\n"
    );

    Ok(())
}
