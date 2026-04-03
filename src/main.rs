mod ast;
mod parser;
mod codegen;
mod compiler;

fn main() {
    let src = r#"
        fn add(a, b) {
            return a + b;
        }

        fn main() {
            let x = add(3, 4);
            print(x);
            return 0;
        }
    "#;

    compiler::compile_and_run(src);
}
