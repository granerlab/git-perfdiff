use std::env::current_dir;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::cli::Args;
use crate::config::file::load as load_config_file;
use crate::config::{Command, Formatter, Validated};
use crate::git::Context as GitContext;
use crate::git::DiffTargets;
use crate::measurement::Results;

use super::Config;

/// Get the current working directory.
fn get_current_dir() -> Result<PathBuf> {
    current_dir().map_err(|err| anyhow!("Unable to get current directory: {err}"))
}

/// Context for a program invocation.
pub struct ExecutionContext<'a> {
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

impl ExecutionContext<'_> {
    /// Create config object from CLI args and config file.
    fn from_config(config: Config) -> Result<Self> {
        /// Default git branch
        const DEFAULT_MAIN_BRANCH: &str = "main";
        /// Default template for printing results.
        const DEFAULT_OUTPUT_TEMPLATE: &str =
            "Ran in {{ wall_time.secs + wall_time.nanos / 1e9 }} s.";
        /// Default value for showing program output.
        const DEFAULT_SHOW_OUTPUT: bool = false;

        // TODO: Allow these to be specified in config file and envvars.
        let show_output = config.show_output.unwrap_or(DEFAULT_SHOW_OUTPUT);
        let git_path = match config.git_path {
            Some(path) => path,
            None => get_current_dir()?,
        };
        let command = config
            .command
            .ok_or_else(|| anyhow!("No command to run!"))?;

        let working_dir = config.working_dir.unwrap_or_else(|| git_path.clone());

        let build_command = config
            .build_command
            .map(|build_command| {
                Command::new(
                    build_command,
                    config.build_arg.unwrap_or_default(),
                    working_dir.clone(),
                    show_output,
                )
                .validate()
            })
            .transpose()?;

        let command = Command::new(
            command,
            config.arg.unwrap_or_default(),
            working_dir,
            show_output,
        )
        .validate()?;

        let git_ctx = GitContext::try_from(git_path)?;

        let default_branch = config
            .main_branch_name
            .as_ref()
            .map_or(DEFAULT_MAIN_BRANCH, |v| v);
        let git_targets = DiffTargets::from_string_refs(
            &git_ctx,
            config.base_git_ref.as_ref().map_or(default_branch, |v| v),
            config.head_git_ref.as_ref().map_or("HEAD", |v| v),
        )?;

        let output_template = config
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
        let config_file = load_config_file(".perfdiff.toml");
        Self::from_config(config_file.overwrite(cli_args.into()))
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
