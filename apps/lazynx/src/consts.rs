use lazy_static::lazy_static;

pub const CONFIG_PATH: &str = include_str!("./.config/config.json5");

lazy_static! {
    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
}
