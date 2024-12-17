use std::{pin::Pin, time::Duration};

use futures::{Stream, StreamExt};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

use crate::event::Event;

pub fn render_stream(frame_rate: f64) -> Pin<Box<dyn Stream<Item = Event>>> {
    let render_delay = Duration::from_secs_f64(1.0 / frame_rate);
    let render_interval = interval(render_delay);

    return Box::pin(IntervalStream::new(render_interval).map(|_| Event::Render));
}
