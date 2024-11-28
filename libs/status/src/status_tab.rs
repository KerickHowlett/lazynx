use ratatui::{prelude::*, widgets::*};

use app_config::Config;
use common::Component;

#[derive(Default)]
pub struct StatusTab {}

impl StatusTab {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_block(&self) -> Block {
        return Block::default()
            .title("[1]—Status")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .padding(Padding::left(3));
    }
}

impl Component<Config> for StatusTab {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let block = self.create_block();

        // TODO: Replace placeholder for dynamic means of acquiring Nx Workspace
        //-      name from the appropriate config file.
        let placeholder = Text::from("LazyNx Workspace");
        let project_name = Paragraph::new(placeholder).left_aligned();

        frame.render_widget(project_name.block(block), area);
    }
}
