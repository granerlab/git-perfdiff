use std::path::PathBuf;

use clap::Parser;

/// Measure performance of a program across git commits.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Command to run
    #[arg(short, long)]
    pub command: String,

    /// Arguments to pass to program
    #[arg(short, long)]
    pub arg: Option<Vec<String>>,

    /// Working directory for program execution
    #[arg(short, long)]
    pub working_dir: Option<PathBuf>,

    /// Whether to show program output
    #[arg(long, action)]
    pub show_output: bool,

    /// Local path to git repository
    // TODO: Make this optional, defaulting to the current directory.
    #[arg(long, short)]
    pub path: PathBuf,

    /// Base commit in comparison
    // TODO: Default to branch split, or root commit.
    // TODO: Should this be an `OsString`?
    #[arg()]
    pub base: Option<String>,

    /// Head commit in comparison
    // TODO: Default to current HEAD.
    #[arg()]
    pub head: Option<String>,
}
