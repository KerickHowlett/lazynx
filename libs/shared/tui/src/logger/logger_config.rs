use std::path::PathBuf;

pub struct LoggerConfig {
    pub project_name: String,
}

impl LoggerConfig {
    pub fn new(project_name: String) -> Self {
        let project_name = project_name.to_uppercase().to_string();
        return Self { project_name };
    }

    pub fn get_data_dir(&self) -> PathBuf {
        return std::env::var(format!("{}_DATA", self.project_name))
            .ok()
            .map(PathBuf::from)
            .unwrap_or(String::from("LAZYNX_DATA").into());
    }

    pub fn get_log_env(&self) -> String {
        return format!("{}_LOG_LEVEL", self.project_name);
    }

    pub fn get_log_file_name(&self) -> String {
        return format!("{}.log", env!("CARGO_PKG_NAME"));
    }
}
