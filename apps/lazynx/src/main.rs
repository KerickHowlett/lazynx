mod app;

use color_eyre::Result;

use app::App;
use app_config::Config;
use crossterm::event::EventStream;
use events::EventLoopHandler;
use shell::AppWidget;
use tui::TuiRunner;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::default();

    logger::init(logger::Config {
        data_dir: config.data_dir.clone(),
        log_level: config.log_level,
    })?;

    let tui = TuiRunner::default()
        .set_draw(true)
        .set_mouse(config.enable_mouse)
        .set_paste(config.enable_paste);

    errors::install_hooks(tui)?;

    let frame_rate = config.frame_rate;
    let event_loop = EventLoopHandler::new(EventStream::new(), frame_rate);

    let mut app = App::<AppWidget>::default();
    let backend = tui.init()?;

    app.run(backend, config, event_loop)?;

    tui.restore()?;

    Ok(())
}
