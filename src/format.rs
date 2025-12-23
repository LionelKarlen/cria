use std::{
    fs::OpenOptions,
    io::{Read, Seek, Write, read_to_string},
    path::PathBuf,
};

use clap::Args;
use miette::{IntoDiagnostic, Result};

use crate::todo::{Todo, TodoKind};

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

fn format_todo_lines(todos: &str) -> String {
    let todos = Todo::from_file_text(todos);
    let mut total = Vec::new();
    let mut marked = Vec::new();
    let mut regular = Vec::new();
    let mut completed = Vec::new();
    for todo in todos {
        match todo.kind {
            TodoKind::Marked => marked.push(todo.value),
            TodoKind::Regular => regular.push(todo.value),
            TodoKind::Completed => completed.push(todo.value),
        }
    }

    if !marked.is_empty() {
        let m = marked.join("\n");
        total.push(m);
        total.push("\n".into());
    }
    if !regular.is_empty() {
        let r = regular.join("\n");
        total.push(r);
    }
    if !completed.is_empty() {
        let c = completed.join("\n");
        total.push("\n\n".into());
        total.push(c);
    }

    total.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_1() {
        let input = "x completed
        . dotted
        regular";
        let output = ". dotted
regular

x completed";
        assert_eq!(format_todo_lines(input), output);
    }

    #[test]
    fn test_format_reverse_dotted() {
        let input = "x completed
        . dotted
        . second dotted
        regular";
        let output = ". second dotted
. dotted
regular

x completed";
        assert_eq!(format_todo_lines(input), output);
    }

    #[test]
    fn test_format_no_dots() {
        let input = "x completed
        regular 2
        regular";
        let output = "regular 2
regular

x completed";
        assert_eq!(format_todo_lines(input), output);
    }
}
