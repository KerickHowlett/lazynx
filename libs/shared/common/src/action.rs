use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize, Hash)]
pub enum Action {
    #[default]
    ClearScreen,
    Error(String),
    Help,
    Init,
    Quit,
    Render,
    Resize {
        x: u16,
        y: u16,
    },
    Resume,
    Suspend,
    Tick,
}
