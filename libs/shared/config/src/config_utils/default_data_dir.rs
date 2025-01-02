use std::{env, path::PathBuf};

use super::project_dirs;

/// Returns the directory to use for storing data files.
pub fn default_data_dir() -> PathBuf {
    return env::var("LAZYNX_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|_| project_dirs().map(|dirs| dirs.data_local_dir().to_path_buf()))
        .unwrap_or(PathBuf::from(".").join(".data"));
}
