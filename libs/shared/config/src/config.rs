use crate::{keybindings::KeyBindings, style::Styles};
use color_eyre::Result;
use directories::ProjectDirs;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{default::Default, env, hash::Hash, path::PathBuf};
use tracing::error;

lazy_static! {
    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
    pub static ref DATA_FOLDER: Option<PathBuf> =
        env::var(format!("{}_DATA", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
    pub static ref CONFIG_FOLDER: Option<PathBuf> =
        env::var(format!("{}_CONFIG", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub data_dir: PathBuf,
    #[serde(default)]
    pub config_dir: PathBuf,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config<TMode, TAction>
where
    TMode: Eq + Hash,
    TAction: Default + Copy + Eq + Hash,
{
    #[serde(default, flatten)]
    pub config: AppConfig,
    #[serde(default, bound = "TMode: Deserialize<'de>, TAction: Deserialize<'de>")]
    pub keybindings: KeyBindings<TMode, TAction>,
    #[serde(default)]
    pub styles: Styles<TMode>,
}

impl<TMode, TAction> Config<TMode, TAction>
where
    TMode: for<'de> Deserialize<'de> + Default + Hash + Eq + Copy,
    TAction: for<'a> Deserialize<'a> + Default + Copy + Eq + Hash,
{
    pub fn new(default_config: &str) -> Result<Self, config::ConfigError> {
        let default_config: Config<TMode, TAction> = json5::from_str(default_config).unwrap();
        let data_dir = get_data_dir();
        let config_dir = get_config_dir();
        let mut builder = config::Config::builder()
            .set_default("data_dir", data_dir.to_str().unwrap())?
            .set_default("config_dir", config_dir.to_str().unwrap())?;

        let config_files = [
            ("config.json5", config::FileFormat::Json5),
            ("config.json", config::FileFormat::Json),
            ("config.yaml", config::FileFormat::Yaml),
            ("config.toml", config::FileFormat::Toml),
            ("config.ini", config::FileFormat::Ini),
        ];

        let mut found_config = false;
        for (file, format) in &config_files {
            let source = config::File::from(config_dir.join(file))
                .format(*format)
                .required(false);
            builder = builder.add_source(source);
            if config_dir.join(file).exists() {
                found_config = true
            }
        }

        if !found_config {
            error!("No configuration file found. Application may not behave as expected");
        }

        let mut config: Self = builder.build()?.try_deserialize()?;
        for (mode, default_bindings) in default_config.keybindings.iter() {
            let user_bindings = config.keybindings.entry(*mode).or_default();
            for (key, cmd) in default_bindings.iter() {
                user_bindings
                    .entry(key.clone())
                    .or_insert_with(|| cmd.clone());
            }
        }
        for (mode, default_styles) in default_config.styles.iter() {
            let user_styles = config.styles.entry(*mode).or_default();
            for (style_key, style) in default_styles.iter() {
                user_styles.entry(style_key.clone()).or_insert(*style);
            }
        }

        Ok(config)
    }
}

pub fn get_config_dir() -> PathBuf {
    let directory = if let Some(s) = CONFIG_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".config")
    };
    directory
}

pub fn get_data_dir() -> PathBuf {
    let directory = if let Some(s) = DATA_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".data")
    };
    directory
}

fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "kerickhowlett", "lazynx")
}
