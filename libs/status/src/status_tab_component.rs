use std::env;

use ratatui::{prelude::*, widgets::*};

use app_config::Config;
use common::Component;

#[derive(Default)]
pub struct StatusTabComponent {
    workspace_name: String,
}

impl StatusTabComponent {
    pub fn new() -> Self {
        return Self::default();
    }

    fn create_block(&self) -> Block {
        return Block::default()
            .title(String::from("[1]—Status"))
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .padding(Padding::left(3));
    }
}

impl Component<Config> for StatusTabComponent {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let block = self.create_block();

        let workspace_name = Text::from(self.workspace_name.clone());
        let paragraph = Paragraph::new(workspace_name).left_aligned().block(block);

        frame.render_widget(paragraph, area);
    }

    fn init(&mut self) -> color_eyre::eyre::Result<()> {
        self.workspace_name = env::current_dir()
            .unwrap()
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| String::from("Unknown Workspace"));

        Ok(())
    }
}
