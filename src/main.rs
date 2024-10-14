use clap::Parser;
use git_perfdiff::{record_runtime, Args, CommandConfig};
fn main() {
    let args = Args::parse();

    let command = CommandConfig::from(&args);

    let measurement = record_runtime(&command);
    println!("Ran in {measurement} seconds.");
}
