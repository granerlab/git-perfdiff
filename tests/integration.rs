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
    git_add(&ctx.repo, &[toml_name])?;

    let build_script_name = Path::new("build.sh");
    let build_script_path = &ctx.path.join(build_script_name);

    let script_name = "script.sh";
    let gitignore = Path::new(".gitignore");
    std::fs::write(ctx.path.join(gitignore), script_name)?;
    git_add(&ctx.repo, &[gitignore])?;

    let script_content = "echo 'hello'";
    std::fs::write(
        build_script_path,
        format!("echo \"{script_content}\" > {script_name}"),
    )
    .with_context(|| format!("Failed to write {build_script_path:#?}"))?;

    git_add(&ctx.repo, &[build_script_name])?;
    let base_sha = git_commit(&ctx.repo, "Added build script")?;

    let sleep_duration = 0.2;

    let new_script_content = format!("sleep {sleep_duration} && {script_content}");
    std::fs::write(
        build_script_path,
        format!("echo \"{new_script_content}\" > {script_name}"),
    )?;

    git_add(&ctx.repo, &[build_script_name])?;
    let head_sha = git_commit(&ctx.repo, "Changed script")?;

    let args = cli::Args {
        command: "/bin/sh".to_string(),
        arg: Some(Vec::from([script_name.to_owned()])),
        build_command: Some("/bin/sh".to_string()),
        build_arg: Some(Vec::from([build_script_name.to_str().unwrap().to_string()])),
        working_dir: None,
        show_output: false,
        path: ctx.path.clone(),
        base: Some(base_sha.to_string()),
        head: Some(head_sha.to_string()),
    };

    let config = Config::from_args(args).expect("Configuration failed to validate");
    let build_command = &config.build_command.unwrap();
    let command_config = &config.command;
    let diff_targets = &config.git_targets;

    ctx.checkout(diff_targets.base_ref.to_string())?;

    build_command.to_command().status()?;
    let measurement = measurement::record_runtime(command_config);
    assert!(measurement < PERFORMANCE_EPSILON);

    assert!(gitignore.exists());
    // std::fs::remove_file(&ctx.path.join(script_name))?;
    ctx.checkout(diff_targets.head_ref.to_string())?;

    build_command.to_command().status()?;
    let measurement = measurement::record_runtime(command_config);
    assert!((measurement - sleep_duration).abs() < PERFORMANCE_EPSILON);
    Ok(())
}
