use crate::cli::Args;
use std::marker::PhantomData;
use std::path::Path;
use std::process::{Command, Stdio};
use which::which;

/// The validation state of a command,
/// used for ensuring command is valid before execution.
pub trait ValidationState {}

/// Represent a not yet validated command.
pub struct NotValidated;

/// Represent a successfully validated command.
pub struct Validated;

impl ValidationState for NotValidated {}
impl ValidationState for Validated {}

/// Everything required to execute an external command.
pub struct Config<'a, S: ValidationState> {
    /// The program to execute.
    pub program: &'a str,
    /// Arguments to be passed to the program.       
    pub args: &'a [String],
    /// The directory where the program is executed.
    pub working_dir: &'a Option<String>,
    /// Whether to print output to stdout.
    pub show_output: bool,
    /// Phantom data to allow the state to be set.
    _marker: PhantomData<S>,
}

impl<'a, S: ValidationState> Config<'a, S> {
    /// Transition the command configuration to another state.
    const fn transition<N: ValidationState>(self) -> Config<'a, N> {
        // TODO: Do this without writing out all struct members.
        Config {
            program: self.program,
            args: self.args,
            working_dir: self.working_dir,
            show_output: self.show_output,
            _marker: PhantomData,
        }
    }
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

#[derive(Debug)]
/// Ways that command validation could fail.
pub enum ValidationFailure {
    /// The command was not found on the PATH.
    CommandNotFound,
    /// The directory to execute does not exist.
    /// Symbolic links are followed in the verification of this.
    WorkingDirNotFound,
}

impl<'a> Config<'a, NotValidated> {
    /// Validate that the command can be executed as configured.
    /// Obviously does not guarantee that the command runs successfully,
    /// but it should at least be possible to start.
    /// Does not validate the arguments passed to the program.
    ///
    /// # Errors
    ///
    /// An error is returned if the command configuration fails the validation.
    pub fn validate(self) -> Result<Config<'a, Validated>, ValidationFailure> {
        if which(self.program).is_err() {
            return Err(ValidationFailure::CommandNotFound);
        }
        match self.working_dir {
            // Follow symlinks and make sure path exists.
            Some(path) if !Path::new(path).try_exists().unwrap_or(false) => {
                Err(ValidationFailure::WorkingDirNotFound)
            }
            _ => Ok(self.transition()),
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
