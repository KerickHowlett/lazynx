use clap::Parser;
use cli::CLI;
use color_eyre::Result;

use crate::{config::Config, runner::Runner};

mod action;
mod app;
mod cli;
mod components;
mod config;
mod errors;
mod logging;
mod runner;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    log::debug!("Starting in main...");
    let args = CLI::parse();
    let (tick_rate, frame_rate) = (args.tick_rate, args.frame_rate);
    let config = Config::new()?;
    let mut app = Runner::new(config, tick_rate, frame_rate)?;

    app.run().await?;
    Ok(())
}
