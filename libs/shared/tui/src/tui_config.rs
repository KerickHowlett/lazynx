use std::io::{stdout, Stdout};

use color_eyre::eyre::Result;
use crossterm::{
    event::{DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

#[derive(Clone, Copy, Default)]
pub struct TuiConfig {
    pub enable_mouse: bool,
    pub enable_paste: bool,
}

#[derive(Clone, Copy, Default)]
pub struct Tui {
    config: TuiConfig,
}

pub type TuiBackend = Terminal<CrosstermBackend<Stdout>>;

impl Tui {
    pub fn new(config: TuiConfig) -> Tui {
        Tui { config }
    }

    pub fn init(self) -> Result<TuiBackend> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        if self.config.enable_mouse {
            execute!(stdout(), EnableMouseCapture)?;
        }
        if self.config.enable_paste {
            execute!(stdout(), EnableBracketedPaste)?;
        }

        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;
        terminal.hide_cursor()?;

        Ok(terminal)
    }

    pub fn restore(self) -> Result<()> {
        if self.config.enable_paste {
            execute!(stdout(), DisableBracketedPaste)?;
        }

        if self.config.enable_mouse {
            execute!(stdout(), DisableMouseCapture)?;
        }

        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}
