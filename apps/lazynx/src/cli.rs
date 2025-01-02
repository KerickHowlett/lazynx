use clap::Parser;

use crate::consts::CURRENT_VERSION;

#[derive(Parser, Debug)]
#[command(version = format!("version {}", CURRENT_VERSION.to_string()), about)]
pub struct CLI {
    /// Tick rate, i.e. number of ticks per second
    #[arg(short, long, value_name = "FLOAT", default_value_t = 4.0)]
    pub tick_rate: f64,

    /// Frame rate, i.e. number of frames per second
    #[arg(short, long, value_name = "FLOAT", default_value_t = 60.0)]
    pub frame_rate: f64,
}
