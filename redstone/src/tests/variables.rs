use crate::tests::try_run;

#[test]
fn test_let_and_print() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        let a = 10;
        let b = a * 2;
        print(b);
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
        "20\n"
    );

    Ok(())
}

#[test]
fn test_reassignment() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        let x = 1;
        x = 2;
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
        "2\n"
    );

    Ok(())
}

#[test]
fn test_typed_variable_i32() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        let x: i32 = 100;
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
        "100\n"
    );

    Ok(())
}

#[test]
fn test_typed_variable_u64() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        let x: u64 = 255;
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
        "255\n"
    );

    Ok(())
}
