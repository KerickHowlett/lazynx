use crossterm::event::Event as CrosstermEvent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Init,
    Quit,
    Error,
    Closed,
    Tick,
    KeyRefresh,
    Render,
    Crossterm(CrosstermEvent),
}
