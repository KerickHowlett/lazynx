use std::rc::Rc;

use ratatui::prelude::{Constraint, Direction, Frame, Layout, Rect};

use workspace::WorkspaceTabWidget;

#[derive(Default)]
pub struct SidebarWidget {
    workspace_tab: WorkspaceTabWidget,
}

impl SidebarWidget {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = self.create_layout(area);

        self.workspace_tab.draw(frame, chunks[0]);
    }

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

#[cfg(test)]
mod sidebar_widget_tests {
    use super::SidebarWidget;

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
    fn test_sidebar_widget_draw() {
        let mut test_bed = TestBed::default();
        test_bed.workspace.setup();

        test_bed.widget.widget.init().unwrap();
        test_bed
            .widget
            .terminal
            .draw(|f| test_bed.widget.widget.draw(f, f.area()))
            .unwrap();

        assert_snapshot!(test_bed.widget.terminal.backend());

        test_bed.workspace.restore();
    }
}
