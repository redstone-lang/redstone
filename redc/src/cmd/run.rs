use std::io;
use std::process::Command;


fn run(output: &str) -> io::Result<()> {
    let out = Command::new(output).output()?;

    print!("{}", String::from_utf8_lossy(&out.stdout));

    Ok(())
}