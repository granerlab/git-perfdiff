//! Compare the performance of two git commits.
use std::marker::PhantomData;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;
use which::which;

/// Module for CLI definition
pub mod cli;

/// The validation state of a command,
/// used for ensuring command is valid before execution.
pub trait CommandState {}

/// Represent a not yet validated command.
pub struct NotValidated;

/// Represent a successfully validated command.
pub struct Validated;

impl CommandState for NotValidated {}
impl CommandState for Validated {}

/// Everything required to execute an external command.
pub struct CommandConfig<'a, S: CommandState> {
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

impl<'a, S: CommandState> CommandConfig<'a, S> {
    /// Transition the command configuration to another state.
    const fn transition<N: CommandState>(self) -> CommandConfig<'a, N> {
        // TODO: Do this without writing out all struct members.
        CommandConfig {
            program: self.program,
            args: self.args,
            working_dir: self.working_dir,
            show_output: self.show_output,
            _marker: PhantomData,
        }
    }
}

impl<'a> From<&'a cli::Args> for CommandConfig<'a, NotValidated> {
    fn from(value: &'a cli::Args) -> Self {
        /// Static variable to use when there are no arguments to pass.
        const EMPTY_ARGS: &[String] = &[];
        let program_args = value
            .arg
            .as_ref()
            .map_or(EMPTY_ARGS, |arg_vec| arg_vec.as_slice());
        CommandConfig {
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
pub enum CommandValidationFailure {
    /// The command was not found on the PATH.
    CommandNotFound,
    /// The directory to execute does not exist.
    /// Symbolic links are followed in the verification of this.
    WorkingDirNotFound,
}

impl<'a> CommandConfig<'a, NotValidated> {
    /// Validate that the command can be executed as configured.
    /// Obviously does not guarantee that the command runs successfully,
    /// but it should at least be possible to start.
    /// Does not validate the arguments passed to the program.
    ///
    /// # Errors
    ///
    /// An error is returned if the command configuration fails the validation.
    pub fn validate(self) -> Result<CommandConfig<'a, Validated>, CommandValidationFailure> {
        if which(self.program).is_err() {
            return Err(CommandValidationFailure::CommandNotFound);
        }
        match self.working_dir {
            // Follow symlinks and make sure path exists.
            Some(path) if !Path::new(path).try_exists().unwrap_or(false) => {
                Err(CommandValidationFailure::WorkingDirNotFound)
            }
            _ => Ok(self.transition()),
        }
    }
}

impl CommandConfig<'_, Validated> {
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

#[must_use]
/// Record the run time of a validated command configuration.
pub fn record_runtime(command: &CommandConfig<Validated>) -> f64 {
    let mut invocation = command.to_command();

    let timer = Instant::now();
    let result = invocation.status();
    let measurement = timer.elapsed().as_secs_f64();

    match result {
        Ok(status) if !status.success() => {
            if let Some(code) = status.code() {
                // TODO: Change to proper logging
                println!("Program exited with code {code}");
            }
        }
        Err(error) => {
            // TODO: Change to proper logging
            println!("Failed with error {error}");
        }
        _ => {}
    }
    measurement
}
