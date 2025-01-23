use std::result;

use color_eyre::eyre::Result;
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use shell::IAppWidget;
use tokio::sync::mpsc::error::TryRecvError;

use app_config::Config;
use events::{Event, EventLoopHandler};
use tui::Tui;

// Key events to quit TUI app.
const QUIT_KEY_C: KeyEvent = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
const QUIT_KEY_D: KeyEvent = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL);

#[derive(Default)]
pub struct App<TShell: IAppWidget> {
    shell: TShell,
    should_quit: bool,
}

impl<TShell: IAppWidget> App<TShell> {
    pub fn run(
        &mut self,
        mut tui: Tui,
        _config: Config,
        mut event_loop: EventLoopHandler,
    ) -> Result<()> {
        self.shell.init()?;

        loop {
            let event = event_loop.next();
            self.event_handler(event, &mut tui)?;

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn draw(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            self.shell.draw(frame, frame.area());
        })?;

        Ok(())
    }

    fn event_handler(
        &mut self,
        event: result::Result<Event, TryRecvError>,
        tui: &mut Tui,
    ) -> Result<()> {
        match event {
            Ok(Event::Render) => self.draw(tui)?,
            Ok(Event::Quit) => self.should_quit = true,
            Ok(Event::Crossterm(CrosstermEvent::Key(key))) => {
                if key == QUIT_KEY_C || key == QUIT_KEY_D {
                    self.should_quit = true;
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(_) => self.should_quit = true,
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod app_tests {
    use crate::app::QUIT_KEY_D;

    use super::{App, QUIT_KEY_C};

    use std::result;

    use app_config::Config;
    use color_eyre::eyre::Result;
    use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
    use pretty_assertions::assert_eq;
    use ratatui::{layout::Rect, Frame};
    use test_case::test_case;
    use tokio::sync::mpsc::error::TryRecvError;

    use events::{Event, EventLoopHandler};
    use shell::IAppWidget;
    use tui::{Tui, TuiRunner};

    #[derive(Default)]
    struct TestShell {
        ran_init: bool,
        is_drawn: bool,
    }

    impl IAppWidget for TestShell {
        fn init(&mut self) -> Result<()> {
            self.ran_init = true;
            Ok(())
        }

        fn draw(&mut self, _frame: &mut Frame, _area: Rect) {
            self.is_drawn = true;
        }
    }

    fn setup() -> Result<(App<TestShell>, Tui, EventLoopHandler)> {
        let mut app = App::<TestShell>::default();
        app.should_quit = false;
        app.shell.is_drawn = false;
        app.shell.ran_init = false;

        let mut tui = TuiRunner::default();
        tui.set_draw(false);
        let backend = tui.init()?;

        const FPS: f64 = 30.0;
        let event_loop = EventLoopHandler::new(tokio_stream::empty(), FPS);

        return Ok((app, backend, event_loop));
    }

    const OTHER_KEY: KeyEvent = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());

    #[test_case(Err(TryRecvError::Disconnected), true, "App should have terminated."; "Disconnected")]
    #[test_case(Ok(Event::Crossterm(CrosstermEvent::Key(QUIT_KEY_C))), true, "App should have terminated."; "Ctrl + C")]
    #[test_case(Ok(Event::Crossterm(CrosstermEvent::Key(QUIT_KEY_D))), true, "App should have terminated."; "Ctrl + D")]
    #[test_case(Ok(Event::Crossterm(CrosstermEvent::Key(OTHER_KEY))), false, "App should not have terminated."; "Any Other Key Should Not Quit")]
    #[tokio::test]
    async fn test_should_quit_events(
        event: result::Result<Event, TryRecvError>,
        expected: bool,
        failure_message: &str,
    ) -> Result<()> {
        let (mut app, mut backend, _) = setup()?;

        app.event_handler(event, &mut backend)?;

        assert_eq!(app.should_quit, expected, "{}", failure_message);

        Ok(())
    }

    #[tokio::test]
    async fn test_render_event_handler() -> Result<()> {
        let (mut app, mut backend, _) = setup()?;

        app.event_handler(Ok(Event::Render), &mut backend)?;

        assert_eq!(
            app.shell.is_drawn, true,
            "App should have rendered view in terminal."
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_run_shell_init() -> Result<()> {
        let (mut app, backend, event_loop) = setup()?;
        app.should_quit = true;

        app.run(backend, Config::default(), event_loop)?;

        assert_eq!(
            app.shell.ran_init, true,
            "App should have called shell's init() method."
        );

        Ok(())
    }
}
