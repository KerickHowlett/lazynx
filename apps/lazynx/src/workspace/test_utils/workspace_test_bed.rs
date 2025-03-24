use std::{env, fmt::Debug, fs, path::PathBuf};

#[derive(Clone, Debug)]
pub struct WorkspaceTestBed {
    pub original_dir: PathBuf,
    pub temp_dir: PathBuf,
    workspace_name: String,
}

impl Default for WorkspaceTestBed {
    fn default() -> Self {
        return WorkspaceTestBed {
            original_dir: PathBuf::new(),
            temp_dir: PathBuf::new(),
            workspace_name: String::from("test"),
        };
    }
}

impl WorkspaceTestBed {
    pub fn get_mock_workspace_name(&self) -> &str {
        return self.workspace_name.as_str();
    }

    pub fn setup(&mut self) {
        self.original_dir = env::current_dir().unwrap();
        self.temp_dir = env::current_dir()
            .unwrap()
            .join(self.workspace_name.clone());

        fs::create_dir_all(self.temp_dir.clone()).unwrap();
        env::set_current_dir(self.temp_dir.clone()).unwrap();
    }

    pub fn restore(&mut self) {
        if self.original_dir.exists() {
            env::set_current_dir(self.original_dir.clone()).unwrap();
        }
        if self.temp_dir.exists() {
            fs::remove_dir_all(self.temp_dir.clone()).unwrap();
        }
    }
}
