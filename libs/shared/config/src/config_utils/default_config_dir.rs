use std::{env, path::PathBuf};

use super::project_dirs;

/// Returns the directory to use for storing config files.
pub fn default_config_dir() -> PathBuf {
    return env::var("LAZYNX_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| project_dirs().map(|dirs| dirs.config_local_dir().to_path_buf()))
        .unwrap_or(PathBuf::from(".").join(".config"));
}
