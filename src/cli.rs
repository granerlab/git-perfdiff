use std::env::current_dir;
use std::ffi::OsString;
use std::path::PathBuf;

use clap::Parser;

/// Get the current working directory.
fn get_current_dir() -> OsString {
    current_dir()
        .expect("Unable to get current directory.")
        .into_os_string()
}

/// Measure performance of a program across git commits.
// TODO: Remove Clone once everything is added to Config
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Command to run
    #[arg(short, long)]
    pub command: String,

    /// Arguments to pass to program
    #[arg(short, long)]
    pub arg: Option<Vec<String>>,

    /// Working directory for program execution. Defaults to the `path` argument.
    #[arg(short, long)]
    pub working_dir: Option<PathBuf>,

    /// Whether to show program output
    #[arg(long, action)]
    pub show_output: bool,

    /// Local path to git repository
    #[arg(long, short, default_value=get_current_dir())]
    pub path: PathBuf,

    /// Base commit in comparison
    // TODO: Default to branch split, or root commit.
    #[arg()]
    pub base: Option<String>,

    /// Head commit in comparison
    #[arg()]
    pub head: Option<String>,
}
