use std::process::Command;
use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target};
use crate::{codegen::Codegen, parser};

pub struct CompileOptions<'a> {
    pub output: &'a str,
    pub target: Option<&'a str>,
}

pub fn compile(src: &str, opts: CompileOptions) -> Result<(), String> {
    Target::initialize_all(&InitializationConfig::default());

    let fns = parser::parse(src).map_err(|e| e.to_string())?;

    let ctx = Context::create();
    let mut cg = Codegen::new(&ctx);
    cg.compile(&fns);

    let obj = format!("{}.o", opts.output);
    cg.emit_object(&obj);

    let mut cmd = Command::new("cc");
    cmd.arg(&obj).arg("-o").arg(opts.output);

    if let Some(triple) = opts.target {
        cmd.arg(format!("--target={triple}"));
    }

    let status = cmd.status().map_err(|e| format!("linker failed: {e}"))?;
    std::fs::remove_file(&obj).ok();

    if status.success() {
        Ok(())
    } else {
        Err(format!("linker exited with {}", status))
    }
}

pub fn compile_and_run(src: &str, output: &str) -> Result<(), String> {
    compile(src, CompileOptions { output, target: None })?;

    let out = Command::new(output)
        .output()
        .map_err(|e| format!("run failed: {e}"))?;

    print!("{}", String::from_utf8_lossy(&out.stdout));

    if out.status.success() {
        Ok(())
    } else {
        Err(format!("program exited with {}", out.status))
    }
}
