use crossterm::event::{KeyEvent, MouseEvent};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Event {
    Closed,
    Error,
    FocusGained,
    FocusLost,
    Init,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(String),
    Quit,
    Render,
    Resize(u16, u16),
    Tick,
}
