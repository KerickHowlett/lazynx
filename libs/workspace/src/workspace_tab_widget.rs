use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::Text,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
};

use crate::workspace_store::{WorkspaceAction, WorkspaceStore};

#[derive(Default, Clone)]
pub struct WorkspaceTabWidget {
    store: WorkspaceStore,
}

impl WorkspaceTabWidget {
    pub fn init(&mut self) -> color_eyre::eyre::Result<()> {
        self.store.update(WorkspaceAction::SetWorkspaceName);

        Ok(())
    }

    fn create_tab(&self) -> Block {
        return Block::default()
            .title("─[1]─Workspace─")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::left(3));
    }
}

impl Widget for WorkspaceTabWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = self.create_tab();
        let workspace_name = Text::from(self.store.get_workspace_name());

        Paragraph::new(workspace_name)
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod workspace_tab_widget_tests {
    use super::WorkspaceTabWidget;

    use insta::assert_snapshot;
    use test_utils::WidgetTestBed;

    use crate::test_bed::WorkspaceTestBed;

    #[derive(Default)]
    struct TestBed {
        widget: WidgetTestBed<WorkspaceTabWidget>,
        workspace: WorkspaceTestBed,
    }

    #[test]
    fn test_draw_widget() {
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
    }
}
