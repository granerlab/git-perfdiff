use crate::cli::Args;
use std::marker::PhantomData;
use std::process::{Command, Stdio};

use super::validation::{NotValidated, State, Validated};

/// Everything required to execute an external command.
pub struct Config<'a, S: State> {
    /// The program to execute.
    pub program: &'a str,
    /// Arguments to be passed to the program.
    pub args: &'a [String],
    /// The directory where the program is executed.
    pub working_dir: &'a Option<String>,
    /// Whether to print output to stdout.
    pub show_output: bool,
    /// Phantom data to allow the state to be set.
    pub(super) _marker: PhantomData<S>,
}

impl<'a> From<&'a Args> for Config<'a, NotValidated> {
    fn from(value: &'a Args) -> Self {
        /// Static variable to use when there are no arguments to pass.
        const EMPTY_ARGS: &[String] = &[];
        let program_args = value
            .arg
            .as_ref()
            .map_or(EMPTY_ARGS, |arg_vec| arg_vec.as_slice());
        Config {
            program: &value.program,
            args: program_args,
            working_dir: &value.working_dir,
            show_output: value.show_output,
            _marker: PhantomData,
        }
    }
}

impl Config<'_, Validated> {
    #[must_use]
    /// Construct an executable command from the configuration.
    pub fn to_command(&self) -> Command {
        let mut command = Command::new(self.program);
        command.args(self.args);
        if let Some(dir) = self.working_dir {
            command.current_dir(dir);
        }
        if !self.show_output {
            command.stdout(Stdio::null());
        }
        command
    }
}
