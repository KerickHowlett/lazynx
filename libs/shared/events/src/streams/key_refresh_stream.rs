use std::{pin::Pin, time::Duration};

use futures::{Stream, StreamExt};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

use crate::event::Event;

pub fn key_refresh_stream(key_refresh_rate: f64) -> Pin<Box<dyn Stream<Item = Event>>> {
    let key_refresh_delay = Duration::from_secs_f64(1.0 / key_refresh_rate);
    let key_refresh_interval = interval(key_refresh_delay);

    return Box::pin(IntervalStream::new(key_refresh_interval).map(|_| Event::KeyRefresh));
}
