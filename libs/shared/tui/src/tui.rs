use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use color_eyre::eyre::Result;
use crossterm::{
    cursor,
    event::{Event as CrosstermEvent, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend as Backend;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use common::Event;

pub type Frame<'a> = ratatui::Frame<'a>;

pub struct Tui {
    pub cancellation_token: CancellationToken,
    pub event_rx: UnboundedReceiver<Event>,
    pub event_tx: UnboundedSender<Event>,
    pub frame_rate: f64,
    pub mouse_enabled: bool,
    pub paste_enabled: bool,
    pub task: JoinHandle<()>,
    pub terminal: ratatui::Terminal<Backend<std::io::Stderr>>,
    pub tick_rate: f64,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let backend = Backend::new(std::io::stderr());
        let terminal = ratatui::Terminal::new(backend)?;

        Ok(Self {
            cancellation_token: CancellationToken::new(),
            event_rx,
            event_tx,
            frame_rate: 60.0,
            mouse_enabled: false,
            paste_enabled: false,
            task: tokio::spawn(async {}),
            terminal,
            tick_rate: 4.0,
        })
    }

    pub fn set_tick_rate(mut self, tick_rate: f64) -> Self {
        self.tick_rate = tick_rate;
        self
    }

    pub fn set_frame_rate(mut self, frame_rate: f64) -> Self {
        self.frame_rate = frame_rate;
        self
    }

    pub fn set_mouse(mut self, mouse: bool) -> Self {
        self.mouse_enabled = mouse;
        self
    }

    pub fn set_paste(mut self, paste: bool) -> Self {
        self.paste_enabled = paste;
        self
    }

    pub fn start(&mut self) {
        let tick_delay = Duration::from_secs_f64(1.0 / self.tick_rate);
        let render_delay = Duration::from_secs_f64(1.0 / self.frame_rate);

        let cancellation_token = self.refresh_cancellation_token();
        let event_tx = self.event_tx.clone();

        self.task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_delay);
            let mut render_interval = tokio::time::interval(render_delay);

            event_tx.send(Event::Init).unwrap();

            loop {
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                  _ = cancellation_token.cancelled() => break,
                  maybe_event = crossterm_event => {
                    match maybe_event {
                      Some(Ok(event)) => {
                        match event {
                          CrosstermEvent::Key(key) => {
                            if key.kind == KeyEventKind::Press {
                              event_tx.send(Event::Key(key)).unwrap();
                            }
                          },
                          CrosstermEvent::Mouse(mouse) => {
                            event_tx.send(Event::Mouse(mouse)).unwrap();
                          },
                          CrosstermEvent::Resize(x, y) => {
                            event_tx.send(Event::Resize(x, y)).unwrap();
                          },
                          CrosstermEvent::FocusLost => {
                            event_tx.send(Event::FocusLost).unwrap();
                          },
                          CrosstermEvent::FocusGained => {
                            event_tx.send(Event::FocusGained).unwrap();
                          },
                          CrosstermEvent::Paste(s) => {
                            event_tx.send(Event::Paste(s)).unwrap();
                          },
                        }
                      }
                      Some(Err(_)) => {
                        event_tx.send(Event::Error).unwrap();
                      }
                      None => {},
                    }
                  },
                  _ = tick_delay => {
                      event_tx.send(Event::Tick).unwrap();
                  },
                  _ = render_delay => {
                      event_tx.send(Event::Render).unwrap();
                  },
                }
            }
        });
    }

    pub fn stop(&self) -> Result<()> {
        self.cancel();

        let mut counter = 0;
        while !self.task.is_finished() {
            std::thread::sleep(Duration::from_millis(1));

            counter += 1;
            if counter > 50 {
                self.task.abort();
            }

            if counter > 100 {
                log::error!("Failed to abort task in 100 milliseconds for unknown reason");
                break;
            }
        }
        Ok(())
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), EnterAlternateScreen, cursor::Hide)?;

        self.start();

        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        self.stop()?;

        if crossterm::terminal::is_raw_mode_enabled()? {
            self.flush()?;
            crossterm::execute!(std::io::stderr(), LeaveAlternateScreen, cursor::Show)?;
            crossterm::terminal::disable_raw_mode()?;
        }

        Ok(())
    }

    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    pub fn suspend(&mut self) -> Result<()> {
        self.exit()?;

        #[cfg(not(windows))]
        signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;

        Ok(())
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }

    fn refresh_cancellation_token(&mut self) -> CancellationToken {
        self.cancellation_token.cancel();
        self.cancellation_token = CancellationToken::new();

        return self.cancellation_token.clone();
    }
}

impl Deref for Tui {
    type Target = ratatui::Terminal<Backend<std::io::Stderr>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}
