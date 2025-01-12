use std::io::{stdout, Stdout};

use color_eyre::eyre::Result;
use crossterm::{
    event::{DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

#[derive(Clone, Copy, Default)]
pub struct Tui {
    enable_mouse: bool,
    enable_paste: bool,
}

pub type TuiBackend = Terminal<CrosstermBackend<Stdout>>;

impl Tui {
    pub fn set_mouse(&mut self, enable_mouse: bool) -> Self {
        self.enable_mouse = enable_mouse;
        return *self;
    }

    pub fn set_paste(&mut self, enable_paste: bool) -> Self {
        self.enable_paste = enable_paste;
        return *self;
    }

    pub fn init(self) -> Result<TuiBackend> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        if self.enable_mouse {
            execute!(stdout(), EnableMouseCapture)?;
        }
        if self.enable_paste {
            execute!(stdout(), EnableBracketedPaste)?;
        }

        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;
        terminal.hide_cursor()?;

        Ok(terminal)
    }

    pub fn restore(self) -> Result<()> {
        if self.enable_paste {
            execute!(stdout(), DisableBracketedPaste)?;
        }

        if self.enable_mouse {
            execute!(stdout(), DisableMouseCapture)?;
        }

        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}
