use std::collections::HashMap;

use super::Config;

/// Exhaustive list of all possible environment variables used for configuration.
static ENVVAR_OPTIONS: &[&str] = &["GIT_MAIN_BRANCH"];

/// Options configurable via environment variables.
struct EnvironmentVariables {
    /// `GIT_MAIN_BRANCH`
    main_branch_name: Option<String>,
}

impl From<EnvironmentVariables> for Config {
    fn from(envvars: EnvironmentVariables) -> Self {
        let EnvironmentVariables { main_branch_name } = envvars;

        Self {
            main_branch_name,
            ..Self::empty()
        }
    }
}

/// Load configuration from environment variables.
#[must_use]
pub fn load() -> Config {
    let envvars: HashMap<String, String> = std::env::vars()
        .filter(|(var, _)| ENVVAR_OPTIONS.contains(&var.as_str()))
        .collect();
    EnvironmentVariables {
        main_branch_name: envvars.get("GIT_MAIN_BRANCH").cloned(),
    }
    .into()
}
