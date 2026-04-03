use std::path::PathBuf;
use clap::{Parser, Subcommand};
use redstone::compiler::{compile, compile_and_run, CompileOptions};

#[derive(Parser)]
#[command(name = "rsc", about = "Redstone compiler")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Compile a .red file into a native executable
    Build {
        /// Source file to compile
        file: PathBuf,

        /// Output executable path
        #[arg(short, long, default_value = "a.out")]
        output: PathBuf,

        /// Target triple (e.g. x86_64-unknown-linux-gnu)
        #[arg(long)]
        target: Option<String>,
    },
    /// Compile and immediately run a .red file
    Run {
        /// Source file to compile and run
        file: PathBuf,
    },
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
