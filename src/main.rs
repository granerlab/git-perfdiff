//! Compare the performance of two git commits.
use std::panic::catch_unwind;

use anyhow::{anyhow, Result};
use clap::Parser;
use git_perfdiff::{
    cli::Args,
    config::Config,
    measurement::{record_runtime, Results},
};

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
            let Results {
                wall_time,
                avg_cpu,
                avg_ram,
            } = record_runtime(command)?;
            println!("Avg cpu usage: {avg_cpu}%");
            println!("Avg mem usage: {ram} kB", ram = avg_ram / 1024);
            println!("Ran in {} seconds.", wall_time.as_secs_f32());
        }
        Ok(())
    });

    // Restore repository to previous state regardless of execution status.
    git_ctx
        .checkout(current_git_ref)
        .expect("Failed to reset repository state after measuring, please inspect manually.");

    program_result.map_err(|_| anyhow!("Internal failure!"))?
}
