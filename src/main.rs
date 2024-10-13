use git_perfdiff::{record_runtime, CommandConfig};

fn main() {
    let program = "sleep";
    let args = &["0.5"];
    let working_dir: Option<&str> = None;

    let command = CommandConfig {
        program,
        args,
        working_dir,
    };

    let measurement = record_runtime(&command);
    println!("Ran in {measurement} seconds.");
}
