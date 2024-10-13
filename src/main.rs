use git_perfdiff::record_runtime;

fn main() {
    let program = "sleep";
    let args = ["0.5"];
    let working_dir: Option<&str> = None;

    let measurement = record_runtime(program, &args, working_dir);
    println!("Ran in {measurement} seconds.");
}
