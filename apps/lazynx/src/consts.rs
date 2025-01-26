use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// Key events to quit TUI app.
pub const QUIT_KEY_CTRL_C: KeyEvent = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
pub const QUIT_KEY_CTRL_D: KeyEvent = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL);
