use std::path::PathBuf;

use clap::Parser;

/// Measure performance of a program across git commits.
// TODO: Remove Clone once everything is added to Config
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Command to run
    #[arg(short, long)]
    pub command: Option<String>,

    /// Arguments to pass to program
    #[arg(short, long)]
    pub arg: Option<Vec<String>>,

    /// Command to run for build step
    #[arg(short('B'), long)]
    pub build_command: Option<String>,

    /// Arguments to pass to build command
    #[arg(short, long)]
    pub build_arg: Option<Vec<String>>,

    /// Working directory for program execution. Defaults to the `path` argument.
    #[arg(short, long)]
    pub working_dir: Option<PathBuf>,

    /// Whether to show program output
    #[arg(long, action)]
    pub show_output: Option<bool>,

    /// Local path to git repository
    #[arg(long, short)]
    pub path: Option<PathBuf>,

    /// Base commit in comparison
    #[arg()]
    pub base: Option<String>,

    /// Head commit in comparison
    #[arg()]
    pub head: Option<String>,
}
