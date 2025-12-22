use std::path::PathBuf;

use clap::Args;

#[derive(Args)]
pub struct ArchiveArgs {
    /// The file from which to archive the completed tasks.
    #[arg(required = true, value_name = "FROM")]
    from: PathBuf,
    /// The file to which the completed tasks should be prepended.
    #[arg(required = true, value_name = "TO")]
    to: PathBuf,
    /// Does not remove the `x` literal denoting a completed task
    #[arg(short, long)]
    no_strip: bool,
    /// The date literal to be prepended to the task. Defaults to `YYYY-MM-DD`
    #[arg(short, long, value_name = "DATE")]
    date: Option<String>,
}
