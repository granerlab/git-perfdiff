use std::process::Command;
use std::time::Instant;

pub struct CommandConfig {
    pub program: &'static str,
    pub args: &'static [&'static str],
    pub working_dir: Option<&'static str>,
}

impl CommandConfig {
    #[must_use]
    pub fn to_command(&self) -> Command {
        let mut command = Command::new(self.program);
        command.args(self.args);
        if let Some(dir) = self.working_dir {
            command.current_dir(dir);
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
