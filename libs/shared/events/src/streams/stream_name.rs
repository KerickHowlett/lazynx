#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum StreamName {
    Ticks,
    KeyRefresh,
    Render,
    Crossterm,
}
