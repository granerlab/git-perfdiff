use super::Config;
use std::{fmt::Display, marker::PhantomData};
///
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandNotFound => f.write_str("Command not found"),
            Self::WorkingDirNotFound => f.write_str("Working directory not found"),
        }
    }
}

impl std::error::Error for Error {}

impl<S: State> Config<S> {
    /// Transition the command configuration to another state.
    pub(super) fn transition<N: State>(self) -> Config<N> {
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
