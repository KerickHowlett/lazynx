use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    ClearScreen,
    Error(String),
    Help,
    Quit,
    Render,
    Resize(u16, u16),
    Resume,
    Suspend,
    Tick,
}
