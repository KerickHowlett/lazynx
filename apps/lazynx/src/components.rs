use color_eyre::eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    config::Config,
    tui::{Event, Frame},
};

pub mod status;

pub trait Component {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        let _ = tx;
        Ok(())
    }

    fn handle_events(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::Key(key_event) => self.handle_key_events(key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_events(mouse_event),
            _ => None,
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        let _ = key;
        None
    }

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Option<Action> {
        let _ = mouse;
        None
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        let _ = config; // to appease clippy
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        let _ = action;
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect);
}
