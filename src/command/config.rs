use crate::cli::Args as CliArgs;
use std::process::{Command, Stdio};
use std::{marker::PhantomData, path::Path};

use super::validation::{NotValidated, State, Validated};

/// Everything required to execute an external command.
pub struct Config<'a, S: State> {
    /// The command to execute.
    pub command: &'a str,
    /// Arguments to be passed to the program.
    pub args: &'a [String],
    /// The directory where the program is executed.
    pub working_dir: &'a Path,
    /// Whether to print output to stdout.
    pub show_output: bool,
    /// Phantom data to allow the state to be set.
    pub(super) _marker: PhantomData<S>,
}

impl<'a> From<&'a CliArgs> for Config<'a, NotValidated> {
    fn from(value: &'a CliArgs) -> Self {
        /// Static variable to use when there are no arguments to pass.
        const EMPTY_ARGS: &[String] = &[];
        let args = value
            .arg
            .as_ref()
            .map_or(EMPTY_ARGS, |arg_vec| arg_vec.as_slice());
        // Use working dir if defined, otherwise the root of the repo path.
        let working_dir = value.working_dir.as_ref().unwrap_or(&value.path).as_path();
        Config {
            command: &value.command,
            args,
            working_dir,
            show_output: value.show_output,
            _marker: PhantomData,
        }
    }
}

impl Config<'_, Validated> {
    #[must_use]
    /// Construct an executable command from the configuration.
    pub fn to_command(&self) -> Command {
        let mut command = Command::new(self.command);
        command.args(self.args);
        command.current_dir(self.working_dir);
        if !self.show_output {
            command.stdout(Stdio::null());
        }
        command
    }
}
