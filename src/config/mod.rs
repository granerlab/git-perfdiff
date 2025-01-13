use anyhow::Result;
use std::path::PathBuf;

/// Configuration for command execution.
mod command;
pub use command::Config as Command;
pub use command::Validated;

/// Configuration for output formatting.
mod output;
pub use output::Formatter;

use crate::git::Context as GitContext;
use crate::measurement::Results;
use crate::{cli::Args, git::DiffTargets};
use serde::Deserialize;

#[cfg(test)]
mod tests;

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
pub struct Config<'a> {
    /// Command execution configuration
    pub command: Command<Validated>,
    /// Command execution configuration
    pub build_command: Option<Command<Validated>>,
    /// Git context
    pub git_ctx: GitContext,
    /// Git references to compare
    pub git_targets: DiffTargets,
    /// Template engine
    template_engine: Formatter<'a>,
}

impl Config<'_> {
    /// Create config object from CLI args and config file.
    fn from_args_and_config_file(cli_args: Args, config_file: ConfigFile) -> Result<Self> {
        /// Default git branch
        const DEFAULT_MAIN_BRANCH: &str = "main";
        /// Default template for printing results.
        const DEFAULT_OUTPUT_TEMPLATE: &str =
            "Ran in {{ wall_time.secs + wall_time.nanos / 1e9 }} s.";

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

        let output_template = config_file
            .output_template
            .unwrap_or_else(|| DEFAULT_OUTPUT_TEMPLATE.to_string());
        let template_engine = Formatter::from_template_string(output_template)?;

        Ok(Self {
            command,
            build_command,
            git_ctx,
            git_targets,
            template_engine,
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

    /// Render program results to a string.
    ///
    /// # Errors
    ///
    /// Surfaces any errors encountered in the templating engine.
    pub fn render_results(&self, results: Results) -> Result<String> {
        self.template_engine.render_results(results)
    }
}
