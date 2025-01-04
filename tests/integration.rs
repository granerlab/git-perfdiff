//! Integration tests
use anyhow::{Context, Result};
use git_perfdiff::{cli, config::Config, measurement};
use std::path::Path;

mod utils;
use utils::{git_add, git_commit, git_init, TestContext};

const PERFORMANCE_EPSILON: f64 = 0.1;

#[test]
fn test_integration() -> Result<()> {
    let test_repo_path = Path::new("/tmp/git-perfdiff/test");
    if test_repo_path.exists() {
        std::fs::remove_dir_all(test_repo_path)?;
    }

    let TestContext(ctx) = &git_init(test_repo_path)?;

    // TODO: Test that specifying a non-existing branch doesn't work.
    let config_file_toml = format!(
        r#"
            working_dir = "{0}"
            main_branch_name = "main"
        "#,
        &ctx.path.to_str().unwrap()
    );
    let toml_name = Path::new(".perfdiff.toml");
    let toml_path = &ctx.path.join(toml_name);
    std::fs::write(toml_path, config_file_toml)
        .with_context(|| format!("Failed to write {toml_path:#?}"))?;

    let script_name = Path::new("script.sh");
    let script_path = Path::join(&ctx.path, script_name);
    std::fs::write(&script_path, "pwd")
        .with_context(|| format!("Failed to write {script_path:#?}"))?;

    git_add(&ctx.repo, &[script_name, toml_name])?;
    let base_sha = git_commit(&ctx.repo, "Added script")?;

    let sleep_duration = 0.2;

    std::fs::write(&script_path, format!("sleep {sleep_duration} && pwd"))?;

    git_add(&ctx.repo, &[script_name])?;
    let head_sha = git_commit(&ctx.repo, "Changed script")?;

    let args = cli::Args {
        command: "/bin/sh".to_string(),
        arg: Some(Vec::from([script_path.to_str().unwrap().to_string()])),
        working_dir: None,
        show_output: false,
        path: ctx.path.clone(),
        base: Some(base_sha.to_string()),
        head: Some(head_sha.to_string()),
    };

    let config = Config::from_args(args).expect("Configuration failed to validate");
    let command_config = &config.command;
    let diff_targets = &config.git_targets;

    ctx.checkout(diff_targets.base_ref.to_string())?;

    let measurement = measurement::record_runtime(command_config);
    assert!(measurement < PERFORMANCE_EPSILON);

    ctx.checkout(diff_targets.head_ref.to_string())?;

    let measurement = measurement::record_runtime(command_config);
    assert!((measurement - sleep_duration).abs() < PERFORMANCE_EPSILON);
    Ok(())
}
