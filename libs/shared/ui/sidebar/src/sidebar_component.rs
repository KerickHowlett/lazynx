use ratatui::{prelude::*, widgets::*};

use app_config::Config;
use common::Component;

#[derive(Default)]
pub struct SidebarComponent {}

impl SidebarComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_layout(&self, area: Rect) -> Rc<[Rect]> {
        return Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(5), Constraint::Fill(1)])
            .split(area);
    }
}

impl Component<Config> for SidebarComponent {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = self.create_layout(frame.size());
        frame.render_widget(chunks, area);
    }
}
