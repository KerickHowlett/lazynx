mod app;
mod terminal;

use std::io;

use crate::app::App;

use anyhow::Result;
use scopeguard::defer;

fn main() -> Result<()> {
    terminal::setup()?;
    defer! { terminal::shutdown(); }

    let mut terminal = terminal::start(io::stdout())?;

    let app = App::new()?;
    loop {
        if app.requires_redraw.get() {
            app.requires_redraw.set(false);
        }

        terminal::draw(&mut terminal, &app)?;

        if app.exit_app.get() {
            break;
        }
    }

    Ok(())
}
