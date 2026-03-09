use clap::{Parser, Subcommand};
use miette::Result;

use crate::cli::format::{FormatArgs, run_format_command};
mod cli;
mod core;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Format the given file with `marked` entries first, regular entries second, completed entries last.
    Format(FormatArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Format(format_args) => run_format_command(format_args),
    }
}
