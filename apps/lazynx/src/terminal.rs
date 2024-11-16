use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::backend::CrosstermBackend;
use std::io::{self, Stdout};

pub trait CLIApp {
    fn draw(&self, f: &mut ratatui::Frame<'_>) -> Result<()>;
    fn is_redraw_required(&self) -> bool;
}

type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub fn draw<T: CLIApp>(terminal: &mut Terminal, app: &T) -> io::Result<()> {
    if app.is_redraw_required() {
        terminal.clear()?;
    }

    terminal.draw(|f| {
        if let Err(e) = app.draw(f) {
            log::error!("failed to draw: {:?}", e);
        }
    })?;

    Ok(())
}

pub fn setup() -> Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;

    Ok(())
}

pub fn shutdown() {
    let leave_screen = io::stdout().execute(LeaveAlternateScreen).map(|_f| ());

    if let Err(e) = leave_screen {
        eprintln!("leave_screen failed:\n{e}");
    }

    let leave_raw_mode = disable_raw_mode();

    if let Err(e) = leave_raw_mode {
        eprintln!("leave_raw_mode failed:\n{e}");
    }
}

pub fn start(buf: Stdout) -> io::Result<Terminal> {
    let backend = CrosstermBackend::new(buf);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}
