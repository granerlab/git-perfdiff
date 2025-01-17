use anyhow::{anyhow, Result};

use crate::config::{Command, Formatter, Validated};
use crate::git::Context as GitContext;
use crate::git::DiffTargets;
use crate::measurement::Results;

use super::Config;

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
    /// Alias for `ExecutionContext::try_from(config)`.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration could not be successfully validated.
    pub fn from_config(config: Config) -> Result<Self> {
        Self::try_from(config)
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

/// Construct an Error with message
/// ```"No value found for `{value_name}`. Ensure default values are initialized."```
fn missing_default_value(value_name: &str) -> impl Fn() -> anyhow::Error + use<'_> {
    move || anyhow!("No value found for `{value_name}`. Ensure default values are initialized.")
}

impl TryFrom<Config> for ExecutionContext<'_> {
    type Error = anyhow::Error;

    fn try_from(config: Config) -> std::result::Result<Self, Self::Error> {
        // TODO: Allow these to be specified in config file and envvars.
        let show_output = config
            .show_output
            .ok_or_else(missing_default_value("show_output"))?;
        let git_path = config
            .git_path
            .ok_or_else(missing_default_value("git_path"))?;
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
            .ok_or_else(missing_default_value("main_branch_name"))?;
        let git_targets = DiffTargets::from_string_refs(
            &git_ctx,
            config
                .base_git_ref
                .as_ref()
                .map_or(default_branch.as_str(), |v| v),
            config.head_git_ref.as_ref().map_or("HEAD", |v| v),
        )?;

        let output_template = config
            .output_template
            .ok_or_else(missing_default_value("output_template"))?;
        let template_engine = Formatter::from_template_string(output_template)?;

        Ok(Self {
            command,
            build_command,
            git_ctx,
            git_targets,
            template_engine,
        })
    }
}
