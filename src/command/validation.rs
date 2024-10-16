use super::config::Config;
use std::marker::PhantomData;
use which::which;
/// The validation state of a command,
/// used for ensuring command is valid before execution.
pub trait State {}

/// Represent a not yet validated command.
pub struct NotValidated;

/// Represent a successfully validated command.
pub struct Validated;

impl State for NotValidated {}
impl State for Validated {}

#[derive(Debug)]
/// Ways that command validation could fail.
pub enum Error {
    /// The command was not found on the PATH.
    CommandNotFound,
    /// The directory to execute does not exist.
    /// Symbolic links are followed in the verification of this.
    WorkingDirNotFound,
}

impl<'a, S: State> Config<'a, S> {
    /// Transition the command configuration to another state.
    const fn transition<N: State>(self) -> Config<'a, N> {
        // TODO: Do this without writing out all struct members.
        Config {
            command: self.command,
            args: self.args,
            working_dir: self.working_dir,
            show_output: self.show_output,
            _marker: PhantomData,
        }
    }
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
    pub fn validate(self) -> Result<Config<'a, Validated>, Error> {
        if which(self.command).is_err() {
            return Err(Error::CommandNotFound);
        }
        if !self.working_dir.try_exists().unwrap_or(false) {
            return Err(Error::WorkingDirNotFound);
        }
        Ok(self.transition())
    }
}
