mod print;
mod cmd;
mod styles;

use clap::Parser;
use redstone::compiler::{compile, compile_and_run, CompileOptions};
use crate::cmd::Cmd;

#[derive(Parser)]
#[command(
    name = "redc",
    version,
    about = "Redstone compiler",
    arg_required_else_help = true,
    styles=styles::styles()
)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Cmd::Build { file, output, target } => {
            let src = std::fs::read_to_string(&file)
                .unwrap_or_else(|e| { eprintln!("error: {e}"); std::process::exit(1) });

            let output_str = output.to_string_lossy();
            let opts = CompileOptions {
                output: &output_str,
                target: target.as_deref(),
            };

            match compile(&src, opts) {
                Ok(()) => println!("Compiled → {}", output.display()),
                Err(e) => { eprintln!("error: {e}"); std::process::exit(1) }
            }
        }
        Cmd::Run { file } => {
            let src = std::fs::read_to_string(&file)
                .unwrap_or_else(|e| { eprintln!("error: {e}"); std::process::exit(1) });

            let tmp = std::env::temp_dir().join("rsc_run_out");
            let tmp_str = tmp.to_string_lossy().to_string();

            match compile_and_run(&src, &tmp_str) {
                Ok(()) => {}
                Err(e) => { eprintln!("error: {e}"); std::process::exit(1) }
            }
        }
    }
}
