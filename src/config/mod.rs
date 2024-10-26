use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

/// Configuration for command execution.
mod command;
pub use command::validation::Validated;
pub use command::Config as Command;

use crate::git::Context as GitContext;
use crate::{cli::Args, git::DiffTargets};
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
    /// Command execution configuration
    pub command: Command<Validated>,
    /// Git context
    pub git_ctx: GitContext,
    /// Git references to compare
    pub git_targets: DiffTargets,
}

impl Config {
    /// Create config object from CLI args and config file.
    fn from_args_and_config_file(cli_args: Args, config_file: ConfigFile) -> Result<Self> {
        /// Default git branch
        // TODO: Get from config
        const DEFAULT_BRANCH: &str = "main";
        let working_dir = cli_args
            .working_dir
            .or(config_file.working_dir)
            .unwrap_or_else(|| CURRENT_DIRECTORY.to_path_buf());
        let command = Command::new(
            cli_args.command,
            cli_args.arg.unwrap_or_default(),
            working_dir,
            cli_args.show_output,
        )
        .validate()?;
        let git_ctx = GitContext::try_from(cli_args.path)?;
        let git_targets = DiffTargets::from_string_refs(
            &git_ctx,
            cli_args.base.as_ref().map_or(DEFAULT_BRANCH, |v| v),
            cli_args.head.as_ref().map_or("HEAD", |v| v),
        )?;
        Ok(Self {
            command,
            git_ctx,
            git_targets,
        })
    }
    /// Create configuration object from CLI arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration could not be successfully validated.
    pub fn from_args(cli_args: Args) -> Result<Self> {
        let config_file = ConfigFile::load();
        Self::from_args_and_config_file(cli_args, config_file)
    }
}
