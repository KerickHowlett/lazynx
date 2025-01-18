use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::{layout::Rect, Frame};

use status::WorkspaceViewWidget;

use crate::sidebar_widget::SidebarWidget;

#[derive(Default)]
pub struct AppWidget {
    sidebar: SidebarWidget,
    status: WorkspaceViewWidget,
}

impl AppWidget {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn draw(&mut self, frame: &mut Frame, _area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Min(38), Constraint::Percentage(75)])
            .split(frame.area());

        self.sidebar.draw(frame, chunks[0]);
        self.status.draw(frame, chunks[1]);
    }

    pub fn init(&mut self) -> color_eyre::eyre::Result<()> {
        self.sidebar.init()?;
        Ok(())
    }
}
