use std::path::PathBuf;
use std::time::Instant;
use redstone::compiler::{compile, CompileOptions};

pub fn run(file: PathBuf) {
    let src = std::fs::read_to_string(&file)
        .unwrap_or_else(|e| { crate::err!("{}", e); std::process::exit(1) });

    let tmp = std::env::temp_dir().join("redc_run_out");
    let tmp_str = tmp.to_string_lossy().to_string();

    let t = Instant::now();
    if let Err(e) = compile(&src, CompileOptions { output: &tmp_str, target: None }) {
        crate::err!("{}", e);
        std::process::exit(1);
    }
    crate::ok!("Compiled", "{} in {:.2}s", tmp.display(), t.elapsed().as_secs_f64());

    let status = std::process::Command::new(&tmp)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .unwrap_or_else(|e| { crate::err!("run failed: {}", e); std::process::exit(1) });

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
