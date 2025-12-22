use clap::{Parser, Subcommand};

use crate::{archive::ArchiveArgs, format::FormatArgs};
mod archive;
mod format;

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
    /// Archive all completed tasks from a given file.
    Archive(ArchiveArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Format(format_args) => todo!(),
        Commands::Archive(archive_args) => todo!(),
    }
}
