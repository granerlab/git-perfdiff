use git_perfdiff::{cli, command, git, measurement};
use std::path::Path;

mod utils;
use utils::{git_add, git_commit, git_init, TestContext};

const PERFORMANCE_EPSILON: f64 = 0.1;

#[test]
fn test_integration() {
    let TestContext(ctx) = &git_init("/tmp/git-perfdiff/test");

    let script_name = Path::new("script.sh");
    let script_path = Path::join(&ctx.path, script_name);
    std::fs::write(&script_path, "pwd")
        .unwrap_or_else(|_| panic!("Failed to write {script_path:#?}"));

    git_add(&ctx.repo, script_name);
    let base_sha = git_commit(&ctx.repo, "Added script");

    let sleep_duration = 0.2;

    std::fs::write(&script_path, format!("sleep {sleep_duration} && pwd")).unwrap();

    git_add(&ctx.repo, script_name);
    let head_sha = git_commit(&ctx.repo, "Changed script");

    let args = cli::Args {
        command: "/bin/sh".to_string(),
        arg: Some(Vec::from([script_path.to_str().unwrap().to_string()])),
        working_dir: None,
        show_output: false,
        path: ctx.path.clone(),
        base: Some(base_sha.to_string()),
        head: Some(head_sha.to_string()),
    };

    let command_config = command::Config::from(&args).validate().unwrap();
    let diff_targets = git::DiffTargets::from(&args);

    ctx.checkout(diff_targets.base_ref).unwrap();

    let measurement = measurement::record_runtime(&command_config);
    assert!(measurement < PERFORMANCE_EPSILON);

    ctx.checkout(diff_targets.head_ref).unwrap();

    let measurement = measurement::record_runtime(&command_config);
    assert!((measurement - sleep_duration).abs() < PERFORMANCE_EPSILON);
}
