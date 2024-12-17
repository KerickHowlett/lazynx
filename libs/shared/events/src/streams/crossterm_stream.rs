use std::pin::Pin;

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEventKind};
use futures::{Stream, StreamExt};

use crate::event::Event;

pub fn crossterm_stream() -> Pin<Box<dyn Stream<Item = Event>>> {
    Box::pin(EventStream::new().fuse().filter_map(|event| async move {
        match event {
            // Ignore key release / repeat events
            Ok(CrosstermEvent::Key(key)) if key.kind == KeyEventKind::Release => None,
            Ok(event) => Some(Event::Crossterm(event)),
            Err(_) => Some(Event::Error),
        }
    }))
}
