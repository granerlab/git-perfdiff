//! Compare the performance of two git commits.
use anyhow::Result;
use std::fmt::Debug;

use clap::Parser;
use git_perfdiff::{
    cli::Args,
    command::Config,
    git::{Context, DiffTargets},
    measurement::record_runtime,
};

/// Print an error to stdout and exit with a failure code.
fn print_error<E: Debug>(error: E) -> ! {
    println!("Error {error:?}");
    std::process::exit(1)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let command = Config::from(&args).validate();
    let git_ctx = Context::try_from(&args)?;
    let diff_targets = DiffTargets::try_from((&args, &git_ctx))?;

    for git_ref in [diff_targets.base_ref, diff_targets.head_ref] {
        println!("Measuring {git_ref}...");
        git_ctx
            .checkout(git_ref.to_string())
            .expect("Checkout failed");
        match &command {
            Ok(command) => {
                let measurement = record_runtime(command);
                println!("Ran in {measurement} seconds.");
            }
            Err(error) => print_error(error),
        }
    }

    Ok(())
}
