use ratatui::{
    layout::Rect,
    prelude::{Constraint, Direction, Layout},
    Frame,
};

use color_eyre::eyre::Result;
use workspace::WorkspaceViewWidget;

use crate::sidebar_widget::SidebarWidget;

pub trait IAppWidget {
    fn draw(&mut self, frame: &mut Frame, area: Rect);
    fn init(&mut self) -> Result<()>;
}

#[derive(Default)]
pub struct AppWidget {
    sidebar: SidebarWidget,
    workspace: WorkspaceViewWidget,
}

impl AppWidget {
    pub fn new() -> Self {
        return Self::default();
    }
}

impl IAppWidget for AppWidget {
    fn draw(&mut self, frame: &mut Frame, _area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Min(38), Constraint::Percentage(75)])
            .split(frame.area());

        self.sidebar.draw(frame, chunks[0]);
        self.workspace.draw(frame, chunks[1]);
    }

    fn init(&mut self) -> color_eyre::eyre::Result<()> {
        self.sidebar.init()?;
        Ok(())
    }
}
