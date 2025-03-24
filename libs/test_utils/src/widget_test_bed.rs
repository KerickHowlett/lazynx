use ratatui::{backend::TestBackend, Terminal};

#[derive(Clone)]
pub struct WidgetTestBed<TWidget: Default> {
    pub terminal: Terminal<TestBackend>,
    pub widget: TWidget,
}

impl<TWidget: Default> Default for WidgetTestBed<TWidget> {
    fn default() -> Self {
        let backend = TestBackend::new(30, 3);

        return WidgetTestBed {
            terminal: Terminal::new(backend).unwrap(),
            widget: TWidget::default(),
        };
    }
}

impl<TWidget: Default> WidgetTestBed<TWidget> {
    pub fn new(width: u16, height: u16) -> Self {
        let backend = TestBackend::new(width, height);

        return WidgetTestBed {
            terminal: Terminal::new(backend).unwrap(),
            widget: TWidget::default(),
        };
    }

    pub fn with_widget(mut self, widget: TWidget) -> Self {
        self.widget = widget;
        return self;
    }
}
