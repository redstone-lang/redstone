use crate::tests::try_run;

#[test]
fn test_comp_operations() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        print(1 > 2);
        print(2 > 1);
        print(1 >= 1);
        print(1 < 1);
        print(1 <= 1);
        print(1 == 1);
        print(1 != 1);
        return 0;
    }
    "#;

    let result = try_run(SRC)?.output().map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr).map_err(|err| err.to_string())?)
    }

    assert_eq!(
        String::from_utf8(result.stdout).map_err(|err| err.to_string())?,
        "0\n1\n1\n0\n1\n1\n0\n"
    );

    Ok(())
}
