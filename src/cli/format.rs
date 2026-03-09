use std::{
    fs::OpenOptions,
    io::{Read, Seek, Write, read_to_string},
    path::PathBuf,
};

use crate::core::format::format_todo_lines;
use clap::Args;
use miette::{IntoDiagnostic, Result};

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

pub fn run_format_command(args: &FormatArgs) -> Result<()> {
    if args.stdio {
        let file_text = read_to_string(std::io::stdin()).into_diagnostic()?;
        let formatted = format_todo_lines(&file_text);
        print!("{}", formatted);
    } else {
        // safe to unwrap `args.file`, because it's mutually exclusive to `args.stdio`
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(args.file.clone().unwrap())
            .into_diagnostic()?;
        let mut file_text = String::new();
        file.read_to_string(&mut file_text).into_diagnostic()?;
        let formatted = format_todo_lines(&file_text);
        file.seek(std::io::SeekFrom::Start(0)).into_diagnostic()?;
        file.write(&formatted.into_bytes()).into_diagnostic()?;
    };
    Ok(())
}
