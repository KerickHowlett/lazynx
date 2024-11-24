use crate::{action::Action, event::Event};
use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{layout::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

/// `Component` is a trait that represents a visual and interactive element of the user interface.
///
/// Implementors of this trait can be registered with the main application loop and will be able to
/// receive events, update state, and be rendered on the screen.
pub trait Component<TConfig> {
    /// Initialize the component with a specified area if necessary.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    /// Register an action handler that can send actions for processing if necessary.
    ///
    /// # Arguments
    ///
    /// * `tx` - An unbounded sender that can send actions.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        let _ = tx;
        Ok(())
    }

    /// Handle incoming events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `event` - An optional event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    fn handle_events(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::Key(key_event) => self.handle_key_events(key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_events(mouse_event),
            _ => None,
        }
    }

    /// Handle key events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `key` - A key event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        let _ = key;
        None
    }

    /// Handle mouse events and produce actions if necessary.
    ///
    /// # Arguments
    ///
    /// * `mouse` - A mouse event to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Option<Action> {
        let _ = mouse;
        None
    }

    /// Register a configuration handler that provides configuration settings if necessary.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration settings.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    fn register_config_handler(&mut self, config: TConfig) -> Result<()> {
        let _ = config; // to appease clippy
        Ok(())
    }

    /// Update the state of the component based on a received action. (REQUIRED)
    ///
    /// # Arguments
    ///
    /// * `action` - An action that may modify the state of the component.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Action>>` - An action to be processed or none.
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        let _ = action;
        Ok(None)
    }

    /// Render the component on the screen. (REQUIRED)
    ///
    /// # Arguments
    ///
    /// * `f` - A frame used for rendering.
    /// # Returns
    ///
    /// * `Result<()>` - An Ok result or an error.
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect);
}
