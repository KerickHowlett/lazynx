use color_eyre::eyre::Result;
use crossterm::event::Event as CrosstermEvent;
use ratatui::widgets::Widget;
use shell::IAppWidget;

use app_config::Config;
use events::{Event, EventLoopHandler};
use tui::Tui;

use crate::{
    app_status::AppStatus,
    consts::{QUIT_KEY_CTRL_C, QUIT_KEY_CTRL_D},
};

#[derive(Default)]
pub struct App<TShell: Widget + IAppWidget + Clone> {
    shell: TShell,
    status: AppStatus,
}

impl<TShell: Widget + IAppWidget + Clone> App<TShell> {
    pub fn run(
        &mut self,
        mut tui: Tui,
        _config: Config,
        mut event_loop: EventLoopHandler,
    ) -> Result<()> {
        self.shell.init()?;

        while self.status == AppStatus::Running {
            if let Ok(event) = event_loop.next() {
                self.event_handler(event, &mut tui)?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            frame.render_widget(self.shell.clone(), frame.area());
        })?;

        Ok(())
    }

    fn event_handler(&mut self, event: Event, tui: &mut Tui) -> Result<()> {
        match event {
            Event::Render | Event::Crossterm(CrosstermEvent::Resize(_, _)) => self.draw(tui)?,
            Event::Quit => self.quit(),
            Event::Crossterm(CrosstermEvent::Key(key)) => {
                if key == QUIT_KEY_CTRL_C || key == QUIT_KEY_CTRL_D {
                    self.quit();
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn quit(&mut self) {
        self.status = AppStatus::Quit;
    }
}

#[cfg(test)]
mod app_tests {
    use super::App;

    use crate::{
        app_status::AppStatus,
        consts::{QUIT_KEY_CTRL_C, QUIT_KEY_CTRL_D},
    };

    use app_config::Config;
    use color_eyre::eyre::Result;
    use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
    use pretty_assertions::assert_eq;
    use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
    use test_case::test_case;

    use events::{Event, EventLoopHandler};
    use shell::IAppWidget;
    use tui::{Tui, TuiRunner};

    const OTHER_KEY: KeyEvent = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());

    #[derive(Default, Clone)]
    struct TestShell {
        ran_init: bool,
    }

    impl IAppWidget for TestShell {
        fn init(&mut self) -> Result<()> {
            self.ran_init = true;
            Ok(())
        }
    }

    impl Widget for TestShell {
        fn render(self, _area: Rect, _buf: &mut Buffer) {}
    }

    fn setup() -> Result<(App<TestShell>, Tui, EventLoopHandler)> {
        let mut app = App::<TestShell>::default();
        app.status = AppStatus::Running;
        app.shell.ran_init = false;

        let mut tui = TuiRunner::default();
        tui.set_draw(false);
        let backend = tui.init()?;

        const FPS: f64 = 30.0;
        let event_loop = EventLoopHandler::new(tokio_stream::empty(), FPS);

        return Ok((app, backend, event_loop));
    }

    #[test_case(Event::Crossterm(CrosstermEvent::Key(QUIT_KEY_CTRL_C)), AppStatus::Quit, "App should have terminated."; "Ctrl + C")]
    #[test_case(Event::Crossterm(CrosstermEvent::Key(QUIT_KEY_CTRL_D)), AppStatus::Quit, "App should have terminated."; "Ctrl + D")]
    #[test_case(Event::Crossterm(CrosstermEvent::Key(OTHER_KEY)), AppStatus::Running, "App should not have terminated."; "Any Other Key Should Not Quit")]
    #[tokio::test]
    async fn test_should_quit_events(
        event: Event,
        expected: AppStatus,
        failure_message: &str,
    ) -> Result<()> {
        let (mut app, mut backend, _) = setup()?;

        app.event_handler(event, &mut backend)?;

        assert_eq!(app.status, expected, "{}", failure_message);

        Ok(())
    }

    // TODO: Can create passable test once switched over to StatefulWidgets,
    //-      where `is_rendered` can be used for assertions.
    // #[tokio::test]
    // async fn test_render_event_handler() -> Result<()> {
    //     let (mut app, mut backend, _) = setup()?;

    //     app.event_handler(Ok(Event::Render), &mut backend)?;

    //     assert_eq!(
    //         app.shell.is_drawn, true,
    //         "App should have rendered view in terminal."
    //     );

    //     Ok(())
    // }

    #[tokio::test]
    async fn test_run_shell_init() -> Result<()> {
        let (mut app, backend, event_loop) = setup()?;
        app.status = AppStatus::Quit;

        app.run(backend, Config::default(), event_loop)?;

        assert_eq!(
            app.shell.ran_init, true,
            "App should have called shell's init() method."
        );

        Ok(())
    }
}
