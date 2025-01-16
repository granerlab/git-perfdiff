/// Configuration for command execution.
mod command;
use std::path::PathBuf;

pub use command::Config as Command;
pub use command::Validated;

/// Configuration for output formatting.
mod output;
pub use output::Formatter;

/// Configuration loaded from file.
mod file;

/// Configuration from CLI.
mod cli;

/// Execution context from configuration.
mod execution_context;
pub use execution_context::ExecutionContext;

#[derive(Default)]
/// Full configuration.
struct Config {
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
    pub fn extend(self, other: Self) -> Self {
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
    pub fn overwrite(self, other: Self) -> Self {
        other.extend(self)
    }
}
