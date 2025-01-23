// Filters need arguments passed by value.
#![allow(clippy::needless_pass_by_value)]
use std::time::Duration;

use minijinja::{value::ViaDeserialize, Environment};

/// Compute an average over a slice of floating point numbers.
fn average(values: Vec<f64>) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    #[allow(clippy::cast_precision_loss)]
    let count = values.len() as f64;
    values.into_iter().sum::<f64>() / count
}

/// Convert bytes to kibibytes.
fn bytes_to_kb(value: f64) -> f64 {
    value / 1024.0
}

/// Convert bytes to mibibytes.
fn bytes_to_mb(value: f64) -> f64 {
    value / (1024.0 * 1024.0)
}

/// Get number of milliseconds in a Duration.
fn as_millis(value: ViaDeserialize<Duration>) -> u128 {
    value.as_millis()
}

/// Get number of seconds in a Duration.
fn as_secs(value: ViaDeserialize<Duration>) -> f64 {
    value.as_secs_f64()
}

/// Add all custom filters to a templating engine.
pub(super) fn add_filters_to_engine(engine: &mut Environment) {
    engine.add_filter("avg", average);
    engine.add_filter("as_millis", as_millis);
    engine.add_filter("as_secs", as_secs);
    engine.add_filter("as_kb", bytes_to_kb);
    engine.add_filter("as_mb", bytes_to_mb);
}
