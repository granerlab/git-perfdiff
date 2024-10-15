use crate::command::{config::Config, validation::Validated};
use std::time::Instant;

#[must_use]
/// Record the run time of a validated command configuration.
pub fn record_runtime(command: &Config<Validated>) -> f64 {
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
