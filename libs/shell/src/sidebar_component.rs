use std::rc::Rc;

use ratatui::prelude::*;

use app_config::Config;
use common::Component;

use status::StatusTabComponent;

#[derive(Default)]
pub struct SidebarComponent {
    status_tab: StatusTabComponent,
}

impl SidebarComponent {
    fn create_layout(&self, area: Rect) -> Rc<[Rect]> {
        return Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .split(area);
    }
}

impl Component<Config> for SidebarComponent {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = self.create_layout(area);

        self.status_tab.draw(frame, chunks[0]);
    }

    fn init(&mut self) -> color_eyre::eyre::Result<()> {
        self.status_tab.init()?;
        Ok(())
    }
}
