mod cli;
mod consts;
mod runner;

use clap::Parser;
use cli::CLI;
use color_eyre::Result;

use app_config::Config;
use shell::AppLayout;
use tui::{
    errors::initialize_panic_handler,
    logger::{initialize_logger, LoggerConfig},
};

use crate::{
    consts::{CONFIG_PATH, PROJECT_NAME},
    runner::Runner,
};

#[tokio::main]
async fn main() -> Result<()> {
    initialize_panic_handler()?;

    let logger_config = LoggerConfig::new(PROJECT_NAME.clone());
    initialize_logger(logger_config)?;

    log::debug!("Starting in main...");
    let args = CLI::parse();

    let config = Config::new(CONFIG_PATH)?;

    let shell = AppLayout::new();

    let mut app = Runner::new(
        config,
        args.tick_rate,
        args.frame_rate,
        vec![Box::new(shell)],
    )?;

    app.run().await?;
    Ok(())
}
