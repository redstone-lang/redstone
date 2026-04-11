use std::fmt::format;
use crate::tests::try_run;

#[test]
fn test_with_cmp() -> Result<(), String> {
    const SRC_IF_BLOCK: &str = r#"
    fn main() {
        let result = 0;
        while result < 10 {
            result += 1;
        }
        print(result);
        return 0;
    }
    "#;

    let result = try_run(SRC_IF_BLOCK)?.output().map_err(|e| format!("program run failed: {e}"))?;
    if !result.status.success() {
        return Err(String::from_utf8(result.stderr)
            .map_err(|err| format!("read utf8 from stderr: {}", err))?)
    }

    assert_eq!(
        String::from_utf8(result.stdout).map_err(|err| format!("read utf8 from stdout: {}", err))?,
        "10\n"
    );

    Ok(())
}