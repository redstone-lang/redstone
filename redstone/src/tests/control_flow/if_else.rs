use crate::tests::try_run;

#[test]
fn test_if_else() -> Result<(), String> {
    const SRC_IF_BLOCK: &str = r#"
    fn main() {
        let result = 0;
        let a = 1;
        let b = 2;
        if b > a {
            result = b;
        } else {
            result = a;
        }
        print(result);
        return 0;
    }
    "#;
    const SRC_ELSE_BLOCK: &str = r#"
    fn main() {
        let result = 0;
        let a = 2;
        let b = 1;
        if b > a {
            result = b;
        } else {
            result = a;
        }
        print(result);
        return 0;
    }
    "#;
    let result = try_run(SRC_IF_BLOCK)?.output().map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr).map_err(|err| err.to_string())?)
    }
    assert_eq!(
        String::from_utf8(result.stdout).map_err(|err| err.to_string())?,
        "2\n"
    );

    let result = try_run(SRC_ELSE_BLOCK)?.output().map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr).map_err(|err| err.to_string())?)
    }
    assert_eq!(
        String::from_utf8(result.stdout).map_err(|err| err.to_string())?,
        "2\n"
    );

    Ok(())
}