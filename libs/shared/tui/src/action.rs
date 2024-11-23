use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    ClearScreen,
    Error(String),
    Help,
    Init,
    Quit,
    Render,
    Resize { x: u16, y: u16 },
    Resume,
    Suspend,
    Tick,
}
