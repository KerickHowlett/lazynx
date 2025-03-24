mod workspace_store;

mod workspace_view_widget;
pub use workspace_view_widget::WorkspaceViewWidget;

mod workspace_tab_widget;
pub use workspace_tab_widget::WorkspaceTabWidget;

#[cfg(test)]
#[path = "./test_utils/workspace_test_utils.mod.rs"]
pub mod test_bed;
