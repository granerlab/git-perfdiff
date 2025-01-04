use anyhow::Result;
use std::path::PathBuf;

/// Configuration for command execution.
mod command;
pub use command::Config as Command;
pub use command::Validated;

use crate::git::Context as GitContext;
use crate::{cli::Args, git::DiffTargets};
use serde::Deserialize;

/// Contains all options that can be set in the config file
#[derive(Deserialize, Default)]
struct ConfigFile {
    /// Working directory for command execution.
    /// Default is the directory where `git_perfdiff` is executed.
    working_dir: Option<PathBuf>,
    /// Main git branch name.
    /// Default is "main"
    main_branch_name: Option<String>,
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
    /// Command execution configuration
    pub build_command: Option<Command<Validated>>,
    /// Git context
    pub git_ctx: GitContext,
    /// Git references to compare
    pub git_targets: DiffTargets,
}

impl Config {
    /// Create config object from CLI args and config file.
    fn from_args_and_config_file(cli_args: Args, config_file: ConfigFile) -> Result<Self> {
        /// Default git branch
        const DEFAULT_MAIN_BRANCH: &str = "main";

        let working_dir = cli_args
            .working_dir
            .or(config_file.working_dir)
            .unwrap_or_else(|| cli_args.path.clone());

        let build_command = cli_args
            .build_command
            .map(|build_command| {
                Command::new(
                    build_command,
                    cli_args.build_arg.unwrap_or_default(),
                    working_dir.clone(),
                    cli_args.show_output,
                )
                .validate()
            })
            .transpose()?;

        let command = Command::new(
            cli_args.command,
            cli_args.arg.unwrap_or_default(),
            working_dir,
            cli_args.show_output,
        )
        .validate()?;

        let git_ctx = GitContext::try_from(cli_args.path)?;

        let default_branch = config_file
            .main_branch_name
            .as_ref()
            .map_or(DEFAULT_MAIN_BRANCH, |v| v);
        let git_targets = DiffTargets::from_string_refs(
            &git_ctx,
            cli_args.base.as_ref().map_or(default_branch, |v| v),
            cli_args.head.as_ref().map_or("HEAD", |v| v),
        )?;

        Ok(Self {
            command,
            build_command,
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
