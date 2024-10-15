//! Compare the performance of two git commits.

/// CLI definition
pub mod cli;

/// Command configuration
pub mod command {
    /// Command config
    pub mod config;
    /// Validation utilities
    pub mod validation;
}

/// Measurement functions
pub mod measurement;
