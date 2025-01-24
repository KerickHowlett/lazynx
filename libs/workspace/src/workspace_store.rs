use std::env;

#[derive(Default, Clone)]
pub struct WorkspaceStore {
    pub workspace_name: String,
}

pub enum WorkspaceAction {
    SetWorkspaceName,
}

impl WorkspaceStore {
    pub fn update(&mut self, action: WorkspaceAction) {
        match action {
            WorkspaceAction::SetWorkspaceName => {
                self.set_workspace_name();
            }
        }
    }

    pub fn set_workspace_name(&mut self) {
        self.workspace_name = env::current_dir()
            .unwrap()
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| String::from("Unknown Workspace"));
    }

    pub fn get_workspace_name(&self) -> String {
        return self.workspace_name.clone();
    }
}

#[cfg(test)]
mod workspace_store_tests {
    use super::{WorkspaceAction, WorkspaceStore};

    use pretty_assertions::assert_eq;

    use crate::test_bed::WorkspaceTestBed;

    #[derive(Default)]
    struct TestBed {
        store: WorkspaceStore,
        workspace: WorkspaceTestBed,
    }

    const WORKSPACE_NAME_FAIL: &str = "Workspace Name Did Not Match.";

    #[test]
    fn test_set_workspace_name() {
        let mut test_bed = TestBed::default();
        test_bed.workspace.setup();
        let expected = test_bed.workspace.get_mock_workspace_name();

        test_bed.store.update(WorkspaceAction::SetWorkspaceName);
        let response = test_bed.store.workspace_name;

        assert_eq!(response, expected, "{}", WORKSPACE_NAME_FAIL);

        test_bed.workspace.restore();
    }

    #[test]
    fn test_get_workspace_name() {
        let mut test_bed = TestBed::default();
        let expected = String::from("Test Workspace");
        test_bed.store.workspace_name = expected.clone();

        let response = test_bed.store.get_workspace_name();

        assert_eq!(response, expected, "{}", WORKSPACE_NAME_FAIL);
    }
}
