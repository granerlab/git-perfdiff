//! Compare the performance of two git commits.
use anyhow::Result;

use clap::Parser;
use git_perfdiff::{
    cli::Args,
    config::Config,
    git::{Context, DiffTargets},
    measurement::record_runtime,
};

fn main() -> Result<()> {
    let args = Args::parse();

    let git_ctx = Context::try_from(&args)?;
    let diff_targets = DiffTargets::try_from((&args, &git_ctx))?;
    let config = Config::from_args(args.clone())?;
    let command = &config.command;

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
