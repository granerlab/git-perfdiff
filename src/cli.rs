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
    pub working_dir: Option<String>,

    /// Whether to show program output
    #[arg(long, action)]
    pub show_output: bool,
}
