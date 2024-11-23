use color_eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

use super::LoggerConfig;

pub fn initialize_logger(config: LoggerConfig) -> Result<()> {
    let directory = config.get_data_dir();
    std::fs::create_dir_all(directory.clone())?;

    let log_file_name = config.get_log_file_name();
    let log_path = directory.join(log_file_name.clone());
    let log_file = std::fs::File::create(log_path)?;

    let log_env = config.get_log_env();
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG")
            .or_else(|_| std::env::var(log_env.clone()))
            .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME"))),
    );

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
