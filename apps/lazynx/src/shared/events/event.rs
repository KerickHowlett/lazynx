use crossterm::event::Event as CrosstermEvent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Event {
    Crossterm(CrosstermEvent),
    Error(String),
    Init,
    Quit,
    Render,
    Tick,
}
