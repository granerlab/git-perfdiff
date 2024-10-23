use crate::cli::Args;

/// Configuration for a program invocation.
pub struct Config {}

impl Config {
    /// Create configuration object from CLI arguments.
    #[must_use]
    pub fn from_args(_cli_args: &Args) -> Self {
        todo!("Populate config from args and read config file");
    }
}
