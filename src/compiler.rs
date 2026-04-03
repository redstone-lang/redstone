use std::process::Command;
use inkwell::context::Context;
use crate::{codegen::Codegen, parser};

pub fn compile_and_run(src: &str) {
    let fns = parser::parse(src);

    let ctx = Context::create();
    let mut cg = Codegen::new(&ctx);
    cg.compile(&fns);
    cg.emit_object("/tmp/redstone_out.o");

    let status = Command::new("cc")
        .args(["/tmp/redstone_out.o", "-o", "/tmp/redstone_out"])
        .status()
        .expect("linker failed");

    if status.success() {
        println!("Compiled → /tmp/redstone_out");
        let out = Command::new("/tmp/redstone_out")
            .output()
            .expect("run failed");
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }
}
