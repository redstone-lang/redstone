pub mod run;
pub mod build;

use std::path::PathBuf;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Cmd {
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