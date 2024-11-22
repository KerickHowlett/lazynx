use color_eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use std::path::PathBuf;

pub trait LoggerConfig {
    fn get_data_dir(&self) -> PathBuf;
    fn get_log_env(&self) -> String {
        return format!("{}_LOG_LEVEL", self.get_project_name());
    }
    fn get_log_level(&self) -> String {
        return format!("{}.log", env!("CARGO_PKG_NAME"));
    }
    fn get_project_name(&self) -> String;
}

pub fn init(config: &impl LoggerConfig) -> Result<()> {
    let directory = config.get_data_dir();
    std::fs::create_dir_all(directory.clone())?;

    let log_level = config.get_log_level().clone();
    let log_path = directory.join(log_level);
    let log_file = std::fs::File::create(log_path)?;
    let env_filter = EnvFilter::builder().with_default_directive(tracing::Level::INFO.into());

    // If the `RUST_LOG` environment variable is set, use that as the default, otherwise use the
    // value of the `LOG_ENV` environment variable. If the `LOG_ENV` environment variable contains
    // errors, then this will return an error.
    let log_env = config.get_log_env().clone();
    let env_filter = env_filter
        .try_from_env()
        .or_else(|_| env_filter.with_env_var(log_env).from_env())?;

    let file_subscriber = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(env_filter);

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .try_init()?;

    Ok(())
}
