use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::*;

use app_config::Config;
use common::{Action, Component};
use status::StatusView;

#[derive(Default)]
pub struct AppLayout {
    status: StatusView,
}

impl AppLayout {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component<Config> for AppLayout {
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        if let KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } = key
        {
            return Some(Action::Quit);
        };
        None
    }

    fn draw(&mut self, frame: &mut Frame, _area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Min(35), Constraint::Percentage(75)])
            .split(frame.area());

        self.status.draw(frame, chunks[1]);
    }
}
