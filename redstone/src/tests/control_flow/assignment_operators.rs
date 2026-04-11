use crate::tests::try_run;

#[test]
fn test_assignment_operators() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        let a = 0;

        a += 5;
        a -= 1;
        a *= 2;
        a /= 4;

        print(a);
        return 0;
    }
    "#;

    let result = try_run(SRC)?.output().map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr).map_err(|err| err.to_string())?)
    }

    assert_eq!(
        String::from_utf8(result.stdout).map_err(|err| err.to_string())?,
        "2\n"
    );

    Ok(())
}
