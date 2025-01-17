use super::Config;
use crate::cli::Args;

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        let Args {
            command,
            arg,
            build_command,
            build_arg,
            working_dir,
            show_output,
            path,
            base,
            head,
        } = args;
        Self {
            command,
            arg,
            build_command,
            build_arg,
            working_dir,
            show_output,
            git_path: path,
            base_git_ref: base,
            head_git_ref: head,
            ..Self::empty()
        }
    }
}
