//! Compare the performance of two git commits.

/// Git functionality. Mostly wraps the verbose git2 crate.
pub mod git;

/// CLI definition
pub mod cli;

/// Configuration for the run
pub mod config;

/// Measurement functions
pub mod measurement;
