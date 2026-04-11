use crate::tests::try_run;

#[test]
fn test_implicit_return() -> Result<(), String> {
    const SRC: &str = r#"
    
    fn hello(a: i32) -> i32 {
        a
    }
    
    fn main() {
        print(hello(123));
        return 0;
    }
    "#;

    let result = try_run(SRC)?.output().map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr).map_err(|err| err.to_string())?)
    }

    assert_eq!(
        String::from_utf8(result.stdout).map_err(|err| err.to_string())?,
        "123\n"
    );

    Ok(())
}
