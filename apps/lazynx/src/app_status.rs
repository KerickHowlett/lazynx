#[derive(Debug, Default, Eq, PartialEq)]
pub enum AppStatus {
    #[default]
    Running,
    Quit,
}
