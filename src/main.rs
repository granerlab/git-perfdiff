use std::fmt::Debug;

use clap::Parser;
use git_perfdiff::{record_runtime, Args, CommandConfig};

fn print_error<E: Debug>(error: E) -> ! {
    println!("Error {error:?}");
    std::process::exit(1)
}

fn main() {
    let args = Args::parse();

    let command = CommandConfig::from(&args).validate();

    match command {
        Ok(command) => {
            let measurement = record_runtime(&command);
            println!("Ran in {measurement} seconds.");
        }
        Err(error) => print_error(error),
    }
}
