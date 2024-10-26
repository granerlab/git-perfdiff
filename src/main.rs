//! Compare the performance of two git commits.
use anyhow::Result;

use clap::Parser;
use git_perfdiff::{cli::Args, config::Config, measurement::record_runtime};

fn main() -> Result<()> {
    let args = Args::parse();

    let config = Config::from_args(args)?;
    let git_ctx = &config.git_ctx;
    let command = &config.command;
    let diff_targets = &config.git_targets;

    for git_ref in [diff_targets.base_ref, diff_targets.head_ref] {
        println!("Measuring {git_ref}...");
        git_ctx
            .checkout(git_ref.to_string())
            .expect("Checkout failed");
        let measurement = record_runtime(command);
        println!("Ran in {measurement} seconds.");
    }

    Ok(())
}
