//! Compare the performance of two git commits.
use std::panic::catch_unwind;

use anyhow::{anyhow, Result};
use clap::Parser;
use git_perfdiff::{
    cli::Args,
    config::{load_config_file, load_envvars, Config, ExecutionContext},
    git::DiffTargets,
    measurement::{record_runtime, Results},
};

/// Safely run the measurements, restoring the git repo on failure.
fn run_safely(
    execution_context: &ExecutionContext,
    git_ref: &String,
    initial_git_ref: &String,
) -> Result<Results> {
    let ExecutionContext {
        git_ctx,
        build_command,
        command,
        ..
    } = execution_context;
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
    let args: Config = Args::parse().into();
    let config_file = load_config_file(".perfdiff.toml");
    let envvars = load_envvars();

    let config = args
        .extend_with(envvars)
        .extend_with(config_file)
        .extend_with(Config::default());
    let execution_context = ExecutionContext::from_config(config)?;
    let ExecutionContext {
        git_ctx,
        git_targets: DiffTargets { base_ref, head_ref },
        ..
    } = &execution_context;

    let current_git_ref = git_ctx
        .repo
        .head()?
        .name()
        .expect("Current git reference is not valid UTF-8")
        .to_string();

    println!("Measuring {base_ref}...");
    let base_results = run_safely(&execution_context, &base_ref.to_string(), &current_git_ref)?;
    println!("{}", execution_context.render_results(base_results)?);

    println!("Measuring {head_ref}...");
    let head_results = run_safely(&execution_context, &head_ref.to_string(), &current_git_ref)?;
    println!("{}", execution_context.render_results(head_results)?);

    Ok(())
}
