mod app;

use color_eyre::Result;

use app::App;
use app_config::Config;
use crossterm::event::EventStream;
use events::EventLoopHandler;
use tui::Tui;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::default();

    logger::init(logger::Config {
        data_dir: config.data_dir.clone(),
        log_level: config.log_level,
    })?;

    let tui = Tui::default()
        .set_mouse(config.enable_mouse)
        .set_paste(config.enable_paste);

    errors::install_hooks(tui)?;

    let backend = tui.init()?;
    let frame_rate = config.frame_rate;
    let event_loop = EventLoopHandler::new(EventStream::new(), frame_rate);

    let mut app = App::new();
    app.run(backend, config, event_loop)?;

    tui.restore()?;

    Ok(())
}
