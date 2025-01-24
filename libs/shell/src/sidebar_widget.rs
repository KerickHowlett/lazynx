use std::rc::Rc;

use ratatui::{
    buffer::Buffer,
    prelude::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use workspace::WorkspaceTabWidget;

#[derive(Default, Clone)]
pub struct SidebarWidget {
    workspace_tab: WorkspaceTabWidget,
}

impl SidebarWidget {
    pub fn init(&mut self) -> color_eyre::eyre::Result<()> {
        self.workspace_tab.init()?;
        Ok(())
    }

    fn create_layout(&self, area: Rect) -> Rc<[Rect]> {
        return Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .split(area);
    }
}

impl Widget for SidebarWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let chunks = self.create_layout(area);
        self.workspace_tab.render(chunks[0], buf);
    }
}

#[cfg(test)]
mod sidebar_widget_tests {
    use super::SidebarWidget;

    use color_eyre::eyre::Result;
    use insta::assert_snapshot;

    use test_utils::WidgetTestBed;
    use workspace::test_bed::WorkspaceTestBed;

    struct TestBed {
        widget: WidgetTestBed<SidebarWidget>,
        workspace: WorkspaceTestBed,
    }

    impl Default for TestBed {
        fn default() -> Self {
            return TestBed {
                workspace: WorkspaceTestBed::default(),
                widget: WidgetTestBed::<SidebarWidget>::new(30, 50),
            };
        }
    }

    #[test]
    fn test_sidebar_widget_render() -> Result<()> {
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
