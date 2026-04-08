#[macro_use]
mod print;
mod cmd;
mod styles;

use clap::Parser;
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
    command: Cmd
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Cmd::Build { file, output, target } => cmd::build::run(file, output, target),
        Cmd::Run { file } => cmd::run::run(file), // tmp
    }
}
