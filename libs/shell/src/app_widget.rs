use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::{Constraint, Direction, Layout},
    widgets::Widget,
};

use color_eyre::eyre::Result;
use workspace::WorkspaceViewWidget;

use crate::sidebar_widget::SidebarWidget;

pub trait IAppWidget {
    fn init(&mut self) -> Result<()>;
}

#[derive(Clone, Default)]
pub struct AppWidget {
    sidebar: SidebarWidget,
    workspace: WorkspaceViewWidget,
}

impl IAppWidget for AppWidget {
    fn init(&mut self) -> color_eyre::eyre::Result<()> {
        self.sidebar.init()?;
        Ok(())
    }
}

impl Widget for AppWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Min(38), Constraint::Percentage(75)])
            .split(area);

        self.sidebar.render(chunks[0], buf);
        self.workspace.render(chunks[1], buf);
    }
}

#[cfg(test)]
mod app_widget_tests {
    use super::{AppWidget, IAppWidget};

    use color_eyre::eyre::Result;
    use insta::assert_snapshot;

    use test_utils::{mocks::MOCK_DATE, WidgetTestBed};
    use workspace::{test_bed::WorkspaceTestBed, WorkspaceViewWidget};

    struct TestBed {
        widget: WidgetTestBed<AppWidget>,
        workspace: WorkspaceTestBed,
    }

    impl Default for TestBed {
        fn default() -> Self {
            let mut widget = AppWidget::default();
            widget.workspace = WorkspaceViewWidget::new(*MOCK_DATE);
            let widget_testbed = WidgetTestBed::<AppWidget>::new(100, 50).with_widget(widget);

            return TestBed {
                workspace: WorkspaceTestBed::default(),
                widget: widget_testbed,
            };
        }
    }

    #[test]
    fn test_app_widget_render() -> Result<()> {
        let mut test_bed = TestBed::default();
        test_bed.workspace.setup();

        test_bed.widget.widget.init().unwrap();
        test_bed
            .widget
            .terminal
            .draw(|f| f.render_widget(test_bed.widget.widget, f.area()))
            .unwrap();

        assert_snapshot!(test_bed.widget.terminal.backend());

        test_bed.workspace.restore();

        Ok(())
    }
}
