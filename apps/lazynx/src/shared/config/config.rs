use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};
use tracing::level_filters::LevelFilter;

use super::utils::{default_config_dir, default_config_file, default_data_dir};

/// Application configuration.
///
/// This is the main configuration struct for the application.
#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// The directory to use for storing application data (logs etc.).
    pub data_dir: PathBuf,

    /// The directory to use for storing application configuration (colors
    /// etc.).
    pub config_home: PathBuf,

    /// The directory to use for storing application configuration (colors
    /// etc.).
    pub config_file: PathBuf,

    /// The log level to use. Valid values are: error, warn, info, debug, trace,
    /// off. The default is info.
    #[serde_as(as = "NoneAsEmptyString")]
    pub log_level: Option<LevelFilter>,

    /// The frame rate to use for rendering the application's UI and animation.
    pub frame_rate: f64,

    /// Enable/Disable mouse support.
    pub enable_mouse: bool,

    /// Enable/Disable clipboard support.
    pub enable_paste: bool,
}

impl Default for Config {
    fn default() -> Self {
        return Self {
            config_file: default_config_file(),
            config_home: default_config_dir(),
            data_dir: default_data_dir(),
            enable_mouse: false,
            enable_paste: false,
            frame_rate: 30.0,
            log_level: None,
        };
    }
}
