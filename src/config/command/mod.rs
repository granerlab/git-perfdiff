use std::marker::PhantomData;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use which::which;

/// Validation for commands
mod validation;

pub use validation::Validated;
use validation::{Error, NotValidated, State};

/// Everything required to execute an external command.
pub struct Config<S: State> {
    /// The command to execute.
    pub command: String,
    /// Arguments to be passed to the program.
    pub args: Vec<String>,
    /// The directory where the program is executed.
    pub working_dir: PathBuf,
    /// Whether to print output to stdout.
    pub show_output: bool,
    /// Phantom data to allow the state to be set.
    pub(super) _marker: PhantomData<S>,
}

impl Config<NotValidated> {
    /// Validate that the command can be executed as configured.
    /// Obviously does not guarantee that the command runs successfully,
    /// but it should at least be possible to start.
    /// Does not validate the arguments passed to the program.
    ///
    /// # Errors
    ///
    /// An error is returned if the command configuration fails the validation.
    pub fn validate(self) -> Result<Config<Validated>, Error> {
        if which(&self.command).is_err() {
            return Err(Error::CommandNotFound);
        }
        if !self.working_dir.try_exists().unwrap_or(false) {
            return Err(Error::WorkingDirNotFound);
        }
        Ok(self.transition())
    }

    /// Create a new config object
    #[must_use]
    pub(crate) const fn new(
        command: String,
        args: Vec<String>,
        working_dir: PathBuf,
        show_output: bool,
    ) -> Self {
        Self {
            command,
            args,
            show_output,
            working_dir,
            _marker: PhantomData,
        }
    }
}

impl Config<Validated> {
    #[must_use]
    /// Construct an executable command from the configuration.
    pub fn to_command(&self) -> Command {
        let mut command = Command::new(&self.command);
        command.args(&self.args);
        command.current_dir(&self.working_dir);
        if !self.show_output {
            command.stdout(Stdio::null());
        }
        command
    }
}
