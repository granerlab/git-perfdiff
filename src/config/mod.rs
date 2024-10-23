use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::cli::Args;
use serde::Deserialize;

#[cfg(test)]
mod test;

/// Current directory as a Path
static CURRENT_DIRECTORY: LazyLock<&Path> = LazyLock::new(|| Path::new("."));

/// Contains all options that can be set in the config file
#[derive(Deserialize, Default)]
struct ConfigFile {
    /// Working directory for command execution.
    working_dir: Option<PathBuf>,
}

impl ConfigFile {
    /// Load config file
    fn load() -> Self {
        std::fs::read_to_string(".perfdiff.toml").map_or_else(
            |_| Self::default(),
            |file| toml::from_str(file.as_str()).unwrap_or_else(|_| Self::default()),
        )
    }
}

/// Configuration for a program invocation.
pub struct Config {
    /// Working directory for command execution
    pub working_dir: PathBuf,
}

impl Config {
    /// Create config object from CLI args and config file.
    fn from_args_and_config_file(cli_args: Args, config_file: ConfigFile) -> Self {
        let working_dir = cli_args
            .working_dir
            .or(config_file.working_dir)
            .unwrap_or_else(|| CURRENT_DIRECTORY.to_path_buf());
        Self { working_dir }
    }
    /// Create configuration object from CLI arguments.
    #[must_use]
    pub fn from_args(cli_args: Args) -> Self {
        let config_file = ConfigFile::load();
        Self::from_args_and_config_file(cli_args, config_file)
    }
}
