mod lexer;
mod parser;
mod llvm;

use inkwell::context::Context;
use std::process::Command;

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

    let fns = parser::parse(src);

    let ctx = Context::create();
    let mut cg = llvm::Codegen::new(&ctx);
    cg.compile(&fns);
    cg.emit_object("/tmp/redstone_out.o");

    let status = Command::new("cc")
        .args(["/tmp/redstone_out.o", "-o", "/tmp/redstone_out"])
        .status()
        .expect("linker failed");

    if status.success() {
        println!("Compiled → /tmp/redstone_out");
        let out = Command::new("/tmp/redstone_out").output().expect("run failed");
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }
}
