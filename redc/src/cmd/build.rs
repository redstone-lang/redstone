use std::path::PathBuf;
use std::time::Instant;
use redstone::compiler::{compile, CompileOptions};

pub fn run(file: PathBuf, output: PathBuf, target: Option<String>) {
    let src = std::fs::read_to_string(&file)
        .unwrap_or_else(|e| { crate::err!("{}", e); std::process::exit(1) });

    let output_str = output.to_string_lossy().to_string();
    let opts = CompileOptions {
        output: &output_str,
        target: target.as_deref(),
    };

    let t = Instant::now();
    match compile(&src, opts) {
        Ok(()) => crate::ok!("Compiled", "{} in {:.2}s", output.display(), t.elapsed().as_secs_f64()),
        Err(e) => { crate::err!("{}", e); std::process::exit(1) }
    }
}
