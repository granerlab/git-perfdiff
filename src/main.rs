//! Compare the performance of two git commits.
use std::panic::catch_unwind;

use anyhow::{anyhow, Result};
use clap::Parser;
use git_perfdiff::{cli::Args, config::Config, measurement::record_runtime};

fn main() -> Result<()> {
    let args = Args::parse();

    let config = Config::from_args(args)?;
    let git_ctx = &config.git_ctx;
    let build_command = &config.build_command;
    let command = &config.command;
    let diff_targets = &config.git_targets;

    let current_git_ref = git_ctx
        .repo
        .head()?
        .name()
        .expect("Current git reference is not valid UTF-8")
        .to_string();

    let program_result = catch_unwind(|| {
        for git_ref in [diff_targets.base_ref, diff_targets.head_ref] {
            println!("Measuring {git_ref}...");
            git_ctx.checkout(git_ref.to_string())?;
            if let Some(build) = build_command {
                build.to_command().status()?;
            }
            let measurement = record_runtime(command);
            println!("Ran in {measurement} seconds.");
        }
        Ok(())
    });

    // Restore repository to previous state regardless of execution status.
    git_ctx.checkout(current_git_ref)?;

    program_result.map_err(|_| anyhow!("Program failure!"))?
}
