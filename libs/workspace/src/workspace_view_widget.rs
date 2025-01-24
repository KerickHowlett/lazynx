use std::rc::Rc;

use ratatui::{
    prelude::{Constraint, Direction, Frame, Layout, Line, Rect, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

#[derive(Default)]
pub struct WorkspaceViewWidget;

impl WorkspaceViewWidget {
    fn create_block(&self) -> Block {
        return Block::default()
            .title(Line::from("Workspace").left_aligned())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2));
    }

    fn create_layout(&self, area: Rect) -> Rc<[Rect]> {
        return Layout::default()
            .direction(Direction::Vertical)
            .constraints(Constraint::from_lengths([10, 5]))
            .split(area);
    }

    fn get_copyright(&self) -> Paragraph {
        let current_year = chrono::Datelike::year(&chrono::Local::now());
        let copyright = Span::from(format!(
            "Copyright {} {} Kerick Howlett",
            String::from('\u{00A9}'),
            current_year
        ));
        return Paragraph::new(copyright);
    }

    fn get_header(&self) -> Paragraph {
        // NOTE: Don't change the whitespace or alignment for ASCII art text.
        //       Any changes to them will be reflected in the app itself.
        let lazynx_title = String::from(
            r#"
 _                     _   _
| |                   | \ | |
| |     __ _ _____   _|  \| |_  __
| |    / _` |_  / | | | . ` \ \/ /
| |___| (_| |/ /| |_| | |\  |>  <
\_____/\__,_/___|\__, \_| \_/_/\_\
                  __/ |
                 |___ /
"#,
        );

        return Paragraph::new(lazynx_title);
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let block = self.create_block();
        let chunks = self.create_layout(block.inner(area));
        frame.render_widget(block, area);

        let header = self.get_header();
        frame.render_widget(header, chunks[0]);

        let copyright = self.get_copyright();
        frame.render_widget(copyright, chunks[1]);
    }
}

#[cfg(test)]
mod workspace_widget_tests {
    use super::WorkspaceViewWidget;

    use insta::assert_snapshot;
    use test_utils::WidgetTestBed;

    #[test]
    fn test_workspace_view_widget() {
        let mut test_bed = WidgetTestBed::<WorkspaceViewWidget>::new(62, 50);

        test_bed
            .terminal
            .draw(|f| test_bed.widget.draw(f, f.area()))
            .unwrap();

        assert_snapshot!(test_bed.terminal.backend());
    }
}
