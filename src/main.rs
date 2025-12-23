use clap::{Parser, Subcommand};
use miette::Result;

use crate::format::{FormatArgs, run_format_command};
mod format;
mod todo;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Format the given file with `marked` entries first, regular entries second, completed entries last.
    /// `marked` entries are sorted in reverse order, the rest is left in file order.
    Format(FormatArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Format(format_args) => run_format_command(format_args),
    }
}
