use std::path::PathBuf;

use clap::Args;

// Mutually exclusive options
#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct FormatArgs {
    /// The file to format
    file: Option<PathBuf>,
    /// Specify that text input comes from stdin, and the sorted content is given in stdout.
    #[arg(short, long)]
    stdio: bool,
}
