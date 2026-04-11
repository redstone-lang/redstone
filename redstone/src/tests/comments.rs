use crate::tests::try_build;

#[test]
fn test_comment_compiled() -> Result<(), String> {
    const SRC: &str = r#"
    // This is a comment
    // This is a comment

    // This is a comment

    // This is another comment
    fn main() {
        return 0;
    }
    "#;
    try_build(SRC)
}

#[test]
fn test_inline_comment() -> Result<(), String> {
    const SRC: &str = r#"
    fn main() {
        return 0; // This is a comment
    }
    "#;
    try_build(SRC)
}
