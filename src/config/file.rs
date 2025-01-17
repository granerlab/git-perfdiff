use super::Config;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Contains all options that can be set in the config file
#[derive(Deserialize, Default)]
struct ConfigFile {
    /// Working directory for command execution.
    /// Default is the directory where `git_perfdiff` is executed.
    working_dir: Option<PathBuf>,
    /// Main git branch name.
    /// Default is "main"
    main_branch_name: Option<String>,
    /// Template for program output.
    /// Default is each measurement on it's own line
    output_template: Option<String>,
}

impl From<ConfigFile> for Config {
    fn from(config_file: ConfigFile) -> Self {
        let ConfigFile {
            working_dir,
            main_branch_name,
            output_template,
        } = config_file;
        Self {
            working_dir,
            main_branch_name,
            output_template,
            ..Self::empty()
        }
    }
}

/// Load configuration file from a given path.
pub fn load(file_path: impl AsRef<Path>) -> Config {
    std::fs::read_to_string(file_path)
        .map_or_else(
            |_| ConfigFile::default(),
            // TODO: Improve error handling on bad config file
            |file| toml::from_str(file.as_str()).unwrap_or_else(|_| ConfigFile::default()),
        )
        .into()
}
