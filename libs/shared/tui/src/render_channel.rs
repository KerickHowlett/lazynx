use color_eyre::eyre::Result;
use tokio::{
    sync::mpsc::UnboundedSender,
    time::{interval, Duration, Interval},
};

use common::Event;

pub struct RenderChannel {
    event_tx: UnboundedSender<Event>,
    render_interval: Interval,
}

impl RenderChannel {
    pub fn new(render_interval: Interval, event_tx: UnboundedSender<Event>) -> Self {
        return Self {
            event_tx,
            render_interval,
        };
    }

    pub async fn print_frame(&mut self) -> Result<()> {
        self.render_interval.tick().await;
        self.event_tx.send(Event::Render)?;
        Ok(())
    }
}

pub fn create_render_channel(
    render_duration: f64,
    event_tx: UnboundedSender<Event>,
) -> RenderChannel {
    let tick_delay = Duration::from_secs_f64(1.0 / render_duration);
    let render_interval = interval(tick_delay);

    return RenderChannel::new(render_interval, event_tx);
}

#[cfg(test)]
mod render_channel_tests {
    use super::create_render_channel;

    use color_eyre::eyre::Result;
    use pretty_assertions::assert_eq;
    use tokio::sync::mpsc::unbounded_channel;

    use common::Event;

    const FRAME_RATE: f64 = 60.0;

    #[tokio::test]
    async fn test_render_channel_emits_events() -> Result<()> {
        let (event_tx, mut event_rx) = unbounded_channel();
        let mut render_channel = create_render_channel(FRAME_RATE, event_tx);

        render_channel.print_frame().await?;

        let event = event_rx.recv().await.unwrap();
        assert_eq!(event, Event::Render);

        Ok(())
    }

    #[tokio::test]
    async fn test_render_channel_timing() -> Result<()> {
        let (event_tx, mut event_rx) = unbounded_channel();
        let mut render_channel = create_render_channel(FRAME_RATE, event_tx);

        let start = std::time::Instant::now();
        render_channel.print_frame().await?;
        let _ = event_rx.recv().await.unwrap();
        let end = std::time::Instant::now();

        let total_runtime = (end - start).as_secs_f64();
        let expected_runtime = 1.0 / FRAME_RATE;
        assert_eq!(
            total_runtime <= expected_runtime,
            true,
            "Render took longer than expected: {total_runtime}ms"
        );

        Ok(())
    }
}
