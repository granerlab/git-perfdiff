//! Compare the performance of two git commits.
use std::panic::catch_unwind;

use anyhow::{anyhow, Result};
use clap::Parser;
use git_perfdiff::{
    cli::Args,
    config::Config,
    git::DiffTargets,
    measurement::{record_runtime, Results},
};

/// Safely run the measurements, restoring the git repo on failure.
fn run_safely(config: &Config, git_ref: &String, initial_git_ref: &String) -> Result<Results> {
    let Config {
        git_ctx,
        build_command,
        command,
        ..
    } = config;
    let program_result = catch_unwind(|| {
        git_ctx.checkout(git_ref)?;
        if let Some(build) = build_command {
            build.to_command().status()?;
        }
        record_runtime(command)
    });

    // Restore repository to previous state regardless of execution status.
    git_ctx
        .checkout(initial_git_ref)
        .expect("Failed to reset repository state after measuring, please inspect manually.");

    program_result.map_err(|_| anyhow!("Internal failure!"))?
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config = Config::from_args(args)?;
    let Config {
        git_ctx,
        git_targets: DiffTargets { base_ref, head_ref },
        ..
    } = &config;

    let current_git_ref = git_ctx
        .repo
        .head()?
        .name()
        .expect("Current git reference is not valid UTF-8")
        .to_string();

    println!("Measuring {base_ref}...");
    let base_results = run_safely(&config, &base_ref.to_string(), &current_git_ref)?;
    println!("{}", config.render_results(base_results)?);

    println!("Measuring {head_ref}...");
    let head_results = run_safely(&config, &head_ref.to_string(), &current_git_ref)?;
    println!("{}", config.render_results(head_results)?);

    Ok(())
}
