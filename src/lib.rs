//! Compare the performance of two git commits.

/// Git functionality. Mostly wraps the verbose git2 crate.
pub mod git;

/// CLI definition
pub mod cli;

/// Command configuration
pub mod command {
    /// Command config
    pub mod config;
    /// Validation utilities
    pub mod validation;

    pub use config::Config;
}

/// Configuration for the run
pub mod config;

/// Measurement functions
pub mod measurement;
