use std::process::Command;
use std::time::Instant;

#[must_use]
pub fn record_runtime(program: &str, args: &[&str], working_dir: Option<&str>) -> f64 {
    let mut invocation = Command::new(program);
    invocation.args(args);
    if let Some(dir) = working_dir {
        invocation.current_dir(dir);
    }

    let timer = Instant::now();
    let result = invocation.status();
    let measurement = timer.elapsed().as_secs_f64();

    match result {
        Ok(status) if !status.success() => {
            if let Some(code) = status.code() {
                println!("Program exited with code {code}");
            }
        }
        Err(error) => {
            println!("Failed with error {error}");
        }
        _ => {}
    }
    measurement
}
