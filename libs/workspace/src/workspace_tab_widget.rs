use std::env;

use ratatui::{
    layout::{Alignment, Rect},
    text::Text,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

#[derive(Default)]
pub struct WorkspaceTabWidget {
    workspace_name: String,
}

impl WorkspaceTabWidget {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let block = self.create_tab();

        let workspace_name = Text::from(self.workspace_name.clone());
        let paragraph = Paragraph::new(workspace_name).left_aligned().block(block);

        frame.render_widget(paragraph, area);
    }

    pub fn init(&mut self) -> color_eyre::eyre::Result<()> {
        self.workspace_name = env::current_dir()
            .unwrap()
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| String::from("Unknown Workspace"));

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

#[cfg(test)]
mod workspace_tab_widget_tests {
    use super::WorkspaceTabWidget;

    use insta::assert_snapshot;
    use pretty_assertions::assert_eq;

    use test_utils::WidgetTestBed;

    #[test]
    fn test_init() {
        let mut test_bed = WidgetTestBed::<WorkspaceTabWidget>::default();

        test_bed.setup();
        let expected = test_bed.temp_dir.file_name().unwrap().to_str().unwrap();

        test_bed.widget.init().unwrap();

        assert_ne!(
            test_bed.widget.workspace_name, "",
            "Workspace name was empty"
        );
        assert_eq!(
            test_bed.widget.workspace_name, expected,
            "Workspace name did not match"
        );

        test_bed.restore();
    }

    #[test]
    fn test_draw_widget() {
        let mut test_bed = WidgetTestBed::<WorkspaceTabWidget>::default();
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
