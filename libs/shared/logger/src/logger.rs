use std::fs::{create_dir_all, File};

use color_eyre::eyre::Result;
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    self, filter::EnvFilter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

use crate::logger_config::LoggerConfig;

pub fn init(config: LoggerConfig) -> Result<()> {
    let directory = config.data_dir.clone();
    create_dir_all(directory.clone())?;

    let log_file = format!("{}.log", env!("CARGO_PKG_NAME"));
    let log_path = directory.join(log_file);
    let log_file = File::create(log_path)?;

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false);

    let env_filter = EnvFilter::from_default_env()
        .add_directive("tokio_util=off".parse().unwrap())
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap())
        .add_directive(config.log_level.unwrap_or(LevelFilter::OFF).into());

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .with(env_filter)
        .init();

    Ok(())
}