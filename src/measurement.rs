use crate::config::{Command, Validated};
use anyhow::{anyhow, Result};
use std::process::ExitStatus;
use std::sync::LazyLock;
use std::thread::sleep;
use std::time::{Duration, Instant};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

/// Refresh config for probe, we only need cpu and memory.
static CPU_AND_MEM: LazyLock<ProcessRefreshKind> =
    LazyLock::new(|| ProcessRefreshKind::nothing().with_cpu().with_memory());

/// A single probe measurement record.
struct ProbeMeasurement {
    /// CPU utilization percentage. Can be over 100 for multi-core processes.
    cpu: f32,
    /// RAM utilization in Bytes.
    ram: u64,
}

/// Measurement results
pub struct Results {
    /// Wall run time of process.
    pub wall_time: Duration,
    /// Average CPU utilization percentage.
    pub avg_cpu: f32,
    /// Average RAM utilization in Bytes.
    pub avg_ram: u64,
}

impl Results {
    /// Perform necessary aggregations on the measurements to create final results.
    fn from_measurements(wall_time: Duration, measurements: Vec<ProbeMeasurement>) -> Self {
        let probe_count = measurements.len();
        let measurement_sums = measurements
            .into_iter()
            .reduce(|m1, m2| ProbeMeasurement {
                cpu: m1.cpu + m2.cpu,
                ram: m1.ram + m2.ram,
            })
            .unwrap();
        #[allow(clippy::cast_precision_loss)]
        let avg_cpu = measurement_sums.cpu / probe_count as f32;
        let avg_ram = measurement_sums.ram / probe_count as u64;
        Self {
            wall_time,
            avg_cpu,
            avg_ram,
        }
    }
}

/// Handle result of process, depending on exit status.
fn handle_command_result(result: Result<ExitStatus, std::io::Error>) {
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
}

/// Record the run time of a validated command configuration.
///
/// # Errors
///
/// Surfaces any internal errors encountered while running the measured program.
/// Note that the measured program failing is not an error.
pub fn record_runtime(command: &Command<Validated>) -> Result<Results> {
    let mut invocation = command.to_command();

    let mut probe = System::new_with_specifics(RefreshKind::nothing().with_processes(*CPU_AND_MEM));
    // TODO: Make this value configurable, with a warning on too short interval.
    let polling_interval = Duration::from_millis(10);
    // Only probe once the minimum time has passed
    let probing_period: u32 = sysinfo::MINIMUM_CPU_UPDATE_INTERVAL
        .as_micros()
        .div_ceil(polling_interval.as_micros())
        .try_into()?;

    // Allocate 5 seconds worth of measurements to start with
    // TODO: Make this configurable
    let initial_capacity = 5_000 / (u32::try_from(polling_interval.as_millis())? * probing_period);
    let mut probe_results: Vec<ProbeMeasurement> = Vec::with_capacity(initial_capacity as usize);

    let timer = Instant::now();

    let mut handle = invocation.spawn()?;
    let pid = Pid::from_u32(handle.id());
    let pid_binding = [pid];
    let probe_update = ProcessesToUpdate::Some(&pid_binding);

    let mut index = 0u32;
    loop {
        match handle.try_wait().transpose() {
            // Process has finished
            Some(result) => {
                let wall_time = timer.elapsed();
                handle_command_result(result);
                return Ok(Results::from_measurements(wall_time, probe_results));
            }
            // Process is still running
            None => {
                if index % probing_period == 0 {
                    probe.refresh_processes_specifics(probe_update, true, *CPU_AND_MEM);
                    let process = probe
                        .process(pid)
                        .ok_or_else(|| anyhow!(format!("Process {pid} not found")))?;
                    probe_results.push(ProbeMeasurement {
                        cpu: process.cpu_usage(),
                        ram: process.memory(),
                    });
                }
            }
        }
        index += 1;
        sleep(polling_interval);
    }
}
