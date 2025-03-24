use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize, Hash)]
pub enum Action {
    Error(String),
    Help,
    Init,
    Quit,
    Render,
    Resize { x: u16, y: u16 },
}
