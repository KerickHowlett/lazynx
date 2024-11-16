use crate::terminal::CLIApp;
use anyhow::Result;
use ratatui::{layout::Rect, widgets::Paragraph};
use std::cell::Cell;

pub struct App {
    pub exit_app: Cell<bool>,
    pub requires_redraw: Cell<bool>,
}

impl App {
    pub fn new() -> Result<Self> {
        let app = Self {
            exit_app: Cell::new(false),
            requires_redraw: Cell::new(true),
        };

        Ok(app)
    }
}

impl CLIApp for App {
    fn draw(&self, f: &mut ratatui::Frame<'_>) -> Result<()> {
        let area = Rect::new(0, 5, f.area().width, 1);
        f.render_widget(Paragraph::new("Hello World!"), area);

        Ok(())
    }

    fn is_redraw_required(&self) -> bool {
        if !self.requires_redraw.get() {
            return false;
        }

        self.requires_redraw.set(false);
        return true;
    }
}
