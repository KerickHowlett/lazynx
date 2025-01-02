use color_eyre::eyre::{eyre, Result};
use directories::ProjectDirs;

/// Returns the project directories.
pub fn project_dirs() -> Result<ProjectDirs> {
    return ProjectDirs::from("rs", "kerickhowlett", "lazynx")
        .ok_or_else(|| eyre!("user home directory not found"));
}
