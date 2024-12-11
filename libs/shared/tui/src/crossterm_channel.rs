#[cfg(test)]
use std::pin::Pin;

#[cfg(not(test))]
use crossterm::event::EventStream;
use crossterm::event::{Event as CrosstermEvent, KeyEventKind};

#[cfg(test)]
use futures::{stream, Stream};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::UnboundedSender;

use common::Event;

pub struct CrosstermChannel {
    #[cfg(test)]
    reader: Pin<Box<dyn Stream<Item = Result<CrosstermEvent, std::io::Error>>>>,
    #[cfg(not(test))]
    reader: EventStream,
    event_tx: UnboundedSender<Event>,
}

impl CrosstermChannel {
    #[cfg(test)]
    pub fn new(event_tx: UnboundedSender<Event>, events: Vec<CrosstermEvent>) -> Self {
        return Self {
            event_tx,
            reader: Box::pin(stream::iter(events.into_iter().map(Ok))),
        };
    }

    #[cfg(not(test))]
    pub fn new(event_tx: UnboundedSender<Event>) -> Self {
        return Self {
            event_tx,
            reader: EventStream::new(),
        };
    }

    pub async fn emit(&mut self) {
        let crossterm_event = self.reader.next().fuse();
        match crossterm_event.await {
            Some(Ok(event)) => match event {
                CrosstermEvent::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        self.event_tx.send(Event::Key(key)).unwrap();
                    }
                }
                CrosstermEvent::Mouse(mouse) => {
                    self.event_tx.send(Event::Mouse(mouse)).unwrap();
                }
                CrosstermEvent::Resize(x, y) => {
                    self.event_tx.send(Event::Resize(x, y)).unwrap();
                }
                CrosstermEvent::FocusLost => {
                    self.event_tx.send(Event::FocusLost).unwrap();
                }
                CrosstermEvent::FocusGained => {
                    self.event_tx.send(Event::FocusGained).unwrap();
                }
                CrosstermEvent::Paste(clipboard) => {
                    self.event_tx.send(Event::Paste(clipboard)).unwrap();
                }
            },
            Some(Err(_)) => {
                self.event_tx.send(Event::Error).unwrap();
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod crossterm_channel_tests {
    use super::CrosstermChannel;

    use color_eyre::eyre::Result;
    use crossterm::event::{
        Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
        MouseEvent, MouseEventKind,
    };
    use pretty_assertions::assert_eq;
    use test_case::test_case;
    use tokio::sync::mpsc::unbounded_channel;

    use common::Event;

    const MOUSE_EVENT: MouseEvent = MouseEvent {
        kind: MouseEventKind::ScrollUp,
        column: 1,
        row: 2,
        modifiers: KeyModifiers::CONTROL,
    };

    const PRESSED_KEY_EVENT: KeyEvent = KeyEvent {
        kind: KeyEventKind::Press,
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::empty(),
        state: KeyEventState::empty(),
    };

    const RELEASED_KEY_EVENT: KeyEvent = KeyEvent {
        kind: KeyEventKind::Release,
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::empty(),
        state: KeyEventState::empty(),
    };

    #[test_case(CrosstermEvent::Key(PRESSED_KEY_EVENT), Event::Key(PRESSED_KEY_EVENT); "Key")]
    #[test_case(CrosstermEvent::Mouse(MOUSE_EVENT), Event::Mouse(MOUSE_EVENT); "Mouse")]
    #[test_case(CrosstermEvent::Resize(1, 2), Event::Resize(1, 2); "Resize")]
    #[test_case(CrosstermEvent::FocusLost, Event::FocusLost; "FocusLost")]
    #[test_case(CrosstermEvent::FocusGained, Event::FocusGained; "FocusGained")]
    #[test_case(CrosstermEvent::Paste("clipboard".to_string()), Event::Paste("clipboard".to_string()); "Paste")]
    #[tokio::test]
    async fn test_handle_standard_events(
        crossterm_event: CrosstermEvent,
        expected_event: Event,
    ) -> Result<()> {
        let (event_tx, mut event_rx) = unbounded_channel();
        let mut crossterm_channel = CrosstermChannel::new(event_tx, vec![crossterm_event]);

        crossterm_channel.emit().await;
        let sent_event = event_rx.recv().await.unwrap();

        assert_eq!(sent_event, expected_event);

        Ok(())
    }

    #[tokio::test]
    async fn test_key_event_emits_only_on_press() -> Result<()> {
        let (event_tx, event_rx) = unbounded_channel();
        let mut crossterm_channel =
            CrosstermChannel::new(event_tx, vec![CrosstermEvent::Key(RELEASED_KEY_EVENT)]);

        crossterm_channel.emit().await;

        assert_eq!(event_rx.is_empty(), true);

        Ok(())
    }
}
