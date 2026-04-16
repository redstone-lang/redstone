use crate::tests::try_run;

#[test]
fn test_bool_type() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        let flag = true;
        let other = false;
        print(flag);
        print(other);
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
fn test_char_type() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        let letter = 'A';
        print(letter);
        return 0;
    }
    "#;

    let result = try_run(SRC)?.output().map_err(|e| format!("program run failed: {e}"))?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr)
            .map_err(|e| format!("read utf8 from stderr: {e}"))?);
    }

    // 'A' prints as numeric code point 65
    assert_eq!(
        String::from_utf8(result.stdout).map_err(|e| format!("read utf8 from stdout: {e}"))?,
        "65\n"
    );

    Ok(())
}

#[test]
fn test_f64_type() -> Result<(), String> {
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
fn test_f32_type() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        let half: f32 = 0.5;
        print(half);
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
        "0.5\n"
    );

    Ok(())
}

#[test]
fn test_i32_type() -> Result<(), String> {
    const SRC: &str = r#"
    fn identity(x: i32) -> i32 {
        return x;
    }

    fn main() {
        let x: i32 = 42;
        print(identity(x));
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
