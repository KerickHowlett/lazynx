use std::path::PathBuf;

use tracing::level_filters::LevelFilter;

pub struct LoggerConfig {
    /// The directory to use for storing application data (logs etc.).
    pub data_dir: PathBuf,

    /// The log level to use. Valid values are: error, warn, info, debug, trace,
    /// off. The default is info.
    pub log_level: Option<LevelFilter>,
}
