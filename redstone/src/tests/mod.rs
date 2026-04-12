use tempfile::NamedTempFile;
use crate::compiler::{compile, CompileOptions};

mod comments;
mod control_flow;
mod expressions;

fn try_build(src: &str) -> Result<(), String> {
    let tmp_file = NamedTempFile::new().map_err(|e| format!("create temp file: {}", e))?;
    let path = tmp_file.path().to_owned().to_str()
        .ok_or("temp file path is not valid UTF-8".to_string())?
        .to_string();
    compile(src, CompileOptions{ output: path.as_str(), target: None })
        .map_err(|e| format!("compilation failed: {}", e))
}
fn try_run(src: &str) -> Result<std::process::Command, String> {
    let tmp_file = NamedTempFile::new()
        .map_err(|e| format!("create temp file: {}", e))?;

    let (_, path_buf) = tmp_file
        .keep()
        .map_err(|e| format!("keep temp file: {}", e))?;
    let path = path_buf.to_str()
        .ok_or("temp file path is not valid UTF-8".to_string())?
        .to_string();

    compile(src, CompileOptions {
        output: path.as_str(),
        target: None,
    }).map_err(|e| format!("compilation failed: {}", e))?;

    Ok(std::process::Command::new(path))
}
