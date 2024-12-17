use std::pin::Pin;

use futures::{Stream, StreamExt};
use tokio_stream::StreamMap;

use crate::{
    event::Event,
    streams::{crossterm_stream, key_refresh_stream, render_stream, tick_stream, StreamName},
};

pub struct Events {
    streams: StreamMap<StreamName, Pin<Box<dyn Stream<Item = Event>>>>,
}

impl Events {
    pub fn new(key_refresh_rate: f64, frame_rate: f64, tick_rate: f64) -> Self {
        Self {
            streams: StreamMap::from_iter([
                (StreamName::Crossterm, crossterm_stream()),
                (StreamName::KeyRefresh, key_refresh_stream(key_refresh_rate)),
                (StreamName::Render, render_stream(frame_rate)),
                (StreamName::Ticks, tick_stream(tick_rate)),
            ]),
        }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.streams.next().await.map(|(_name, event)| event)
    }
}
