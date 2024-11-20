use std::path::PathBuf;

pub trait LoggerConfig {
    fn get_data_dir(&self) -> PathBuf;
    fn get_log_env(&self) -> String {
        return format!("{}_LOG_LEVEL", self.get_project_name());
    }
    fn get_log_level(&self) -> String {
        return format!("{}.log", env!("CARGO_PKG_NAME"));
    }
    fn get_project_name(&self) -> String;
}
