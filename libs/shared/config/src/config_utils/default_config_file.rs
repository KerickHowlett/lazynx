use std::path::PathBuf;

use super::default_config_dir;

/// Returns the path to the default configuration file.
pub fn default_config_file() -> PathBuf {
    return default_config_dir().join("config.toml");
}
