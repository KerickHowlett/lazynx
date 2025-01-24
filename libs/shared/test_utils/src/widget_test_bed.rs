use std::{env, fs, path::PathBuf};

use ratatui::{backend::TestBackend, Terminal};

pub struct WidgetTestBed<TWidget: Default> {
    pub original_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub terminal: Terminal<TestBackend>,
    pub widget: TWidget,
}

impl<TWidget: Default> Default for WidgetTestBed<TWidget> {
    fn default() -> Self {
        let backend = TestBackend::new(30, 3);

        return WidgetTestBed {
            original_dir: PathBuf::new(),
            temp_dir: PathBuf::new(),
            terminal: Terminal::new(backend).unwrap(),
            widget: TWidget::default(),
        };
    }
}

impl<TWidget: Default> WidgetTestBed<TWidget> {
    pub fn new(width: u16, height: u16) -> Self {
        let backend = TestBackend::new(width, height);

        return WidgetTestBed {
            original_dir: PathBuf::new(),
            temp_dir: PathBuf::new(),
            terminal: Terminal::new(backend).unwrap(),
            widget: TWidget::default(),
        };
    }

    pub fn setup(&mut self) {
        self.original_dir = env::current_dir().unwrap();
        self.temp_dir = env::current_dir().unwrap().join("test");

        fs::create_dir(self.temp_dir.clone()).unwrap();
        env::set_current_dir(self.temp_dir.clone()).unwrap();
    }

    pub fn restore(&mut self) {
        env::set_current_dir(self.original_dir.clone()).unwrap();
        fs::remove_dir_all(self.temp_dir.clone()).unwrap();
    }
}
