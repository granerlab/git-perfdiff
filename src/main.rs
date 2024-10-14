use clap::Parser;
use git_perfdiff::{record_runtime, CommandConfig};

/// Measure performance of a program across git commits.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Program to run
    #[arg(short, long)]
    program: String,

    /// Arguments to pass to program
    #[arg(short, long)]
    arg: Option<Vec<String>>,

    /// Working directory for program execution
    #[arg(short, long)]
    working_dir: Option<String>,

    /// Whether to show program output
    #[arg(long, action)]
    show_output: bool,
}

fn main() {
    let args = Args::parse();

    let command = CommandConfig {
        program: &args.program,
        args: &args.arg.unwrap_or(Vec::new()),
        working_dir: &args.working_dir,
        show_output: args.show_output,
    };

    let measurement = record_runtime(&command);
    println!("Ran in {measurement} seconds.");
}
