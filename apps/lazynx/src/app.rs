use color_eyre::eyre::Result;
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::mpsc::error::TryRecvError;

use app_config::Config;
use events::{Event, EventLoopHandler};
use tui::Tui;

use shell::AppWidget;

pub struct App {
    shell: AppWidget,
    should_quit: bool,
}

const QUIT_KEY: KeyEvent = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);

impl App {
    pub fn new() -> Self {
        return Self {
            shell: AppWidget::new(),
            should_quit: false,
        };
    }

    pub fn run(
        &mut self,
        mut tui: Tui,
        _config: Config,
        mut event_loop: EventLoopHandler,
    ) -> Result<()> {
        self.shell.init()?;

        loop {
            match event_loop.next() {
                Ok(Event::Render) => self.draw(&mut tui)?,
                Ok(Event::Quit) => self.should_quit = true,
                Ok(Event::Crossterm(CrosstermEvent::Key(key))) => {
                    if key == QUIT_KEY {
                        self.should_quit = true;
                    }
                }
                Err(TryRecvError::Disconnected) => self.should_quit = true,

                Err(TryRecvError::Empty) => {}
                _ => {}
            }

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
}
