//! Compare the performance of two git commits.
use std::fmt::Debug;

use clap::Parser;
use git_perfdiff::cli::Args;
use git_perfdiff::command::Config;
use git_perfdiff::measurement::record_runtime;

/// Print an error to stdout and exit with a failure code.
fn print_error<E: Debug>(error: E) -> ! {
    println!("Error {error:?}");
    std::process::exit(1)
}

fn main() {
    let args = Args::parse();

    let command = Config::from(&args).validate();

    match command {
        Ok(command) => {
            let measurement = record_runtime(&command);
            println!("Ran in {measurement} seconds.");
        }
        Err(error) => print_error(error),
    }
}
