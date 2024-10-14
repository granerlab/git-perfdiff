use clap::Parser;
use std::process::{Command, Stdio};
use std::time::Instant;

/// Measure performance of a program across git commits.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Program to run
    #[arg(short, long)]
    pub program: String,

    /// Arguments to pass to program
    #[arg(short, long)]
    pub arg: Option<Vec<String>>,

    /// Working directory for program execution
    #[arg(short, long)]
    pub working_dir: Option<String>,

    /// Whether to show program output
    #[arg(long, action)]
    pub show_output: bool,
}

pub struct CommandConfig<'a> {
    pub program: &'a str,
    pub args: &'a [String],
    pub working_dir: &'a Option<String>,
    pub show_output: bool,
}

impl<'a> From<&'a Args> for CommandConfig<'a> {
    fn from(value: &'a Args) -> Self {
        const EMPTY_ARGS: &[String] = &[];
        let program_args = value
            .arg
            .as_ref()
            .map_or(EMPTY_ARGS, |arg_vec| arg_vec.as_slice());
        CommandConfig {
            program: &value.program,
            args: program_args,
            working_dir: &value.working_dir,
            show_output: value.show_output,
        }
    }
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
