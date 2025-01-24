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

#[cfg(test)]
mod app_widget_tests {
    use super::{AppWidget, IAppWidget};

    use insta::assert_snapshot;

    use test_utils::WidgetTestBed;

    #[test]
    fn test_app_widget_draw() {
        let mut test_bed = WidgetTestBed::<AppWidget>::new(100, 100);
        test_bed.setup();

        test_bed.widget.init().unwrap();
        test_bed
            .terminal
            .draw(|f| test_bed.widget.draw(f, f.area()))
            .unwrap();

        assert_snapshot!(test_bed.terminal.backend());

        test_bed.restore();
    }
}
