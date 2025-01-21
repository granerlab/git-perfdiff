use std::env::current_dir;
use std::path::PathBuf;

/// Configuration for command execution.
mod command;

pub use command::Config as Command;
pub use command::Validated;

/// Configuration for output formatting.
mod output;
pub use output::Formatter;

/// Configuration loaded from file.
mod file;
pub use file::load as load_config_file;

/// Configuration from CLI.
mod cli;

/// Configuration from environment variables.
mod envvars;
pub use envvars::load as load_envvars;

/// Execution context from configuration.
mod execution_context;
pub use execution_context::ExecutionContext;

/// Get the current working directory.
fn get_current_dir() -> Option<PathBuf> {
    current_dir().map_or_else(
        |err| {
            // TODO: Proper logging (warning level)
            println!("Unable to get current directory: {err}");
            None
        },
        Some,
    )
}

/// Full configuration.
#[derive(Clone)]
pub struct Config {
    /// Command to run
    pub command: Option<String>,

    /// Arguments to pass to program
    pub arg: Option<Vec<String>>,

    /// Command to run for build step
    pub build_command: Option<String>,

    /// Arguments to pass to build command
    pub build_arg: Option<Vec<String>>,

    /// Working directory for program execution. Defaults to the `path` argument.
    pub working_dir: Option<PathBuf>,

    /// Whether to show program output
    pub show_output: Option<bool>,

    /// Local path to git repository
    pub git_path: Option<PathBuf>,

    /// Base commit in comparison
    pub base_git_ref: Option<String>,

    /// Head commit in comparison
    pub head_git_ref: Option<String>,

    /// Main git branch name.
    /// Default is "main"
    pub main_branch_name: Option<String>,

    /// Template for program output.
    /// Default is each measurement on it's own line
    pub output_template: Option<String>,
}

impl Config {
    /// Extend configuration by filling missing values with
    /// those from another config object.
    #[must_use]
    pub fn extend_with(self, other: Self) -> Self {
        Self {
            command: self.command.or(other.command),
            arg: self.arg.or(other.arg),
            build_command: self.build_command.or(other.build_command),
            build_arg: self.build_arg.or(other.build_arg),
            working_dir: self.working_dir.or(other.working_dir),
            show_output: self.show_output.or(other.show_output),
            git_path: self.git_path.or(other.git_path),
            base_git_ref: self.base_git_ref.or(other.base_git_ref),
            head_git_ref: self.head_git_ref.or(other.head_git_ref),
            main_branch_name: self.main_branch_name.or(other.main_branch_name),
            output_template: self.output_template.or(other.output_template),
        }
    }

    /// Overwrite configuration with existing values
    /// from another config object.
    #[must_use]
    pub fn overwrite_with(self, other: Self) -> Self {
        other.extend_with(self)
    }

    /// Empty configuration object.
    const fn empty() -> Self {
        Self {
            command: None,
            arg: None,
            build_command: None,
            build_arg: None,
            working_dir: None,
            show_output: None,
            git_path: None,
            base_git_ref: None,
            head_git_ref: None,
            main_branch_name: None,
            output_template: None,
        }
    }
}

impl Default for Config {
    /// Create the default configuration.
    // TODO: Default `base_git_ref` to branch split, or root commit.
    // NOTE: Working dir is defaulted to git_path when constructing exe ctx.
    fn default() -> Self {
        Self {
            show_output: Some(false),
            git_path: get_current_dir(),
            head_git_ref: Some("HEAD".to_string()),
            main_branch_name: Some("main".to_string()),
            output_template: Some(
                "Ran in {{ wall_time.secs + wall_time.nanos / 1e9 }} s.".to_string(),
            ),
            ..Self::empty()
        }
    }
}
