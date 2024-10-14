use std::process::{Command, Stdio};
use std::time::Instant;

pub struct CommandConfig<'a> {
    pub program: &'a String,
    pub args: &'a Vec<String>,
    pub working_dir: &'a Option<String>,
    pub show_output: bool,
}

impl CommandConfig<'_> {
    #[must_use]
    pub fn to_command(&self) -> Command {
        let mut command = Command::new(self.program);
        command.args(self.args);
        if let Some(dir) = self.working_dir {
            command.current_dir(dir);
        }
        if !self.show_output {
            command.stdout(Stdio::null());
        }
        command
    }
}

#[must_use]
pub fn record_runtime(command: &CommandConfig) -> f64 {
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
