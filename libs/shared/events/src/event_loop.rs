use std::{io::Error, pin::Pin, time::Duration};

use crossterm::event::{Event as CrosstermEvent, KeyEventKind};
use futures::Stream;
use tokio::{
    sync::mpsc::{self, error::TryRecvError, UnboundedReceiver, UnboundedSender},
    time::{self, Interval},
};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

use crate::event::Event;

struct EventLoop<TEvents: Stream<Item = Result<CrosstermEvent, Error>> + Send + 'static> {
    cancellation_token: CancellationToken,
    terminal_events: Pin<Box<TEvents>>,
    render_interval: Interval,
    tx: UnboundedSender<Event>,
}

const ERROR_MESSAGE: &str = "Error reading terminal events";

impl<TEvents: Stream<Item = Result<CrosstermEvent, Error>> + Send + 'static> EventLoop<TEvents> {
    fn new(
        frame_rate: f64,
        terminal_events: TEvents,
        tx: UnboundedSender<Event>,
        cancellation_token: CancellationToken,
    ) -> Self {
        let render_delay = Duration::from_secs_f64(1.0 / frame_rate);

        return Self {
            tx,
            cancellation_token,
            terminal_events: Box::pin(terminal_events),
            render_interval: time::interval(render_delay),
        };
    }

    fn terminal_event_handler(&self, event: Result<CrosstermEvent, Error>) {
        if matches!(event, Ok(CrosstermEvent::Resize(_, _))) {
            println!("RESIZE");
        }

        let event = match event {
            Ok(CrosstermEvent::Key(key)) if key.kind == KeyEventKind::Release => None,
            Ok(event) => Some(Event::Crossterm(event)),
            Err(_) => Some(Event::Error(ERROR_MESSAGE.to_string())),
        };

        if let Some(event) = event {
            let _ = self.tx.send(event);
        }
    }

    async fn run(mut self) {
        loop {
            tokio::select! {
                _ = self.cancellation_token.cancelled() => {
                    let _= self.tx.send(Event::Quit);
                    break;
                },
                _ = self.render_interval.tick() => {
                    let _ = self.tx.send(Event::Render);
                },
                Some(terminal_event) = self.terminal_events.next() => {
                    self.terminal_event_handler(terminal_event);
                }
            }
        }
    }
}

pub struct EventLoopHandler {
    rx: UnboundedReceiver<Event>,
    cancellation_token: CancellationToken,
}

impl EventLoopHandler {
    pub fn new<TEvents: Stream<Item = Result<CrosstermEvent, Error>> + Send + 'static>(
        terminal_events: TEvents,
        frame_rate: f64,
    ) -> Self {
        let cancellation_token = CancellationToken::new();
        let (tx, rx) = mpsc::unbounded_channel();

        let event_loop =
            EventLoop::new(frame_rate, terminal_events, tx, cancellation_token.clone());
        tokio::spawn(event_loop.run());

        return Self {
            rx,
            cancellation_token,
        };
    }

    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    pub fn next(&mut self) -> Result<Event, TryRecvError> {
        self.rx.try_recv()
    }
}

#[cfg(test)]
mod event_loop_tests {
    use super::{EventLoopHandler, ERROR_MESSAGE};

    use std::io::{Error, ErrorKind};

    use crossterm::event::{
        Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
    };
    use pretty_assertions::assert_eq;
    use tokio::{
        sync::mpsc::error::TryRecvError,
        time::{sleep, Duration},
    };
    use tokio_stream as stream;

    use crate::event::Event;

    fn setup(events: Vec<Result<CrosstermEvent, Error>>) -> EventLoopHandler {
        const FPS: f64 = 1.0;
        let terminal_events = stream::iter(events.into_iter());

        return EventLoopHandler::new(terminal_events, FPS);
    }

    #[tokio::test(start_paused = true)]
    async fn test_event_loop_terminal_events_key_press() {
        const PRESSED_KEY_EVENT: KeyEvent = KeyEvent {
            kind: KeyEventKind::Press,
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::empty(),
            state: KeyEventState::empty(),
        };
        const EVENT: CrosstermEvent = CrosstermEvent::Key(PRESSED_KEY_EVENT);
        let mut handler = setup(vec![Ok(EVENT.clone())]);

        sleep(Duration::from_secs(1)).await;

        let mut response: Option<Event> = None;
        loop {
            match handler.next() {
                Ok(Event::Render) => {}
                Ok(event) => {
                    response = Some(event);
                    break;
                }
                Err(_) => break,
            }
        }
        handler.cancel();

        assert_eq!(
            response,
            Some(Event::Crossterm(EVENT)),
            "Received unexpected event: {response:?}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_event_loop_terminal_events_key_release() {
        const RELEASED_KEY_EVENT: KeyEvent = KeyEvent {
            kind: KeyEventKind::Release,
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::empty(),
            state: KeyEventState::empty(),
        };
        const EVENT: CrosstermEvent = CrosstermEvent::Key(RELEASED_KEY_EVENT);
        let mut handler = setup(vec![Ok(EVENT.clone())]);

        sleep(Duration::from_secs(1)).await;

        let mut response: Option<Event> = None;
        loop {
            match handler.next() {
                Ok(Event::Render) => {}
                Ok(event) => {
                    response = Some(event);
                    break;
                }
                Err(_) => break,
            }
        }
        handler.cancel();

        assert_eq!(response, None, "There should be no events: {response:?}");
    }

    #[tokio::test(start_paused = true)]
    async fn test_event_loop_terminal_events_error() {
        let error = Error::new(ErrorKind::Other, String::from("error"));
        let mut handler = setup(vec![Err(error)]);

        sleep(Duration::from_secs(1)).await;

        let mut response: Option<Event> = None;
        loop {
            match handler.next() {
                Ok(Event::Render) => {}
                Ok(event) => {
                    response = Some(event);
                    break;
                }
                Err(_) => break,
            }
        }
        handler.cancel();

        assert_eq!(
            response,
            Some(Event::Error(ERROR_MESSAGE.to_string())),
            "Received unexpected event: {response:?}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_event_loop_render_event() {
        let mut handler = setup(vec![]);

        sleep(Duration::from_secs(1)).await;

        let mut received_render_event = false;
        loop {
            match handler.next() {
                Ok(Event::Render) => {
                    received_render_event = true;
                    break;
                }
                Ok(_) => break,
                Err(_) => break,
            }
        }
        handler.cancel();

        assert_eq!(
            received_render_event, true,
            "Failed to receive Render Event."
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_event_loop_send_quit_event_on_cancel() {
        let mut handler = setup(vec![]);

        handler.cancel();
        sleep(Duration::from_secs(1)).await;

        let mut received_quit_event = false;
        loop {
            match handler.next() {
                Ok(Event::Quit) => {
                    received_quit_event = true;
                    break;
                }
                Err(TryRecvError::Empty) => {}
                Err(_) => break,
                _ => {}
            }
        }
        handler.cancel();

        assert_eq!(received_quit_event, true, "Failed to receive Quit Event.");
    }
}
