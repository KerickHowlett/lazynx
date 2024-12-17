use std::{pin::Pin, time::Duration};

use futures::{Stream, StreamExt};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

use crate::event::Event;

pub fn tick_stream(tick_rate: f64) -> Pin<Box<dyn Stream<Item = Event>>> {
    let tick_delay = Duration::from_secs_f64(tick_rate);
    let tick_interval = interval(tick_delay);

    return Box::pin(IntervalStream::new(tick_interval).map(|_| Event::Tick));
}
