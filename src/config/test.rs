use super::*;

fn test_args() -> Args {
    Args {
        command: "pwd".to_string(),
        arg: None,
        working_dir: None,
        show_output: false,
        path: ".".to_string().into(),
        base: None,
        head: None,
    }
}

#[test]
fn test_working_dir() {
    let args = test_args();
    let working_dir = "/work_dir";
    let config_file_toml = format!(
        r#"
            working_dir = "{working_dir}"
        "#
    );
    let config = Config::from_args_and_config_file(
        args,
        toml::from_str(&config_file_toml).expect("Incorrect TOML"),
    );

    assert!(config.working_dir.to_str() == Some(working_dir));
}
