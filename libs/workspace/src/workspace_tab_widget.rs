use std::env;

use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
pub struct WorkspaceTabWidget {
    workspace_name: String,
}

impl WorkspaceTabWidget {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let block = self.create_block();

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

    fn create_block(&self) -> Block {
        return Block::default()
            .title("[1] Workspace")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::left(3));
    }
}
