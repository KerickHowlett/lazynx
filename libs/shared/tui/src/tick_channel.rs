use color_eyre::eyre::Result;
use tokio::{
    sync::mpsc::UnboundedSender,
    time::{interval, Duration, Interval},
};

use common::Event;

pub struct TickChannel {
    event_tx: UnboundedSender<Event>,
    tick_interval: Interval,
}

impl TickChannel {
    pub fn new(tick_interval: Interval, event_tx: UnboundedSender<Event>) -> Self {
        return Self {
            event_tx,
            tick_interval,
        };
    }

    pub async fn tick(&mut self) -> Result<()> {
        self.tick_interval.tick().await;
        self.event_tx.send(Event::Tick)?;
        Ok(())
    }
}

pub fn create_tick_channel(tick_duration: f64, event_tx: UnboundedSender<Event>) -> TickChannel {
    let tick_delay = Duration::from_secs_f64(1.0 / tick_duration);
    let tick_interval = interval(tick_delay);

    return TickChannel::new(tick_interval, event_tx);
}

#[cfg(test)]
mod tick_channel_tests {
    use super::create_tick_channel;

    use color_eyre::eyre::Result;
    use pretty_assertions::assert_eq;
    use tokio::sync::mpsc::unbounded_channel;

    use common::Event;

    const TICK_DELAY: f64 = 4.0;

    #[tokio::test]
    async fn test_tick_channel_emits_events() -> Result<()> {
        let (event_tx, mut event_rx) = unbounded_channel();
        let mut tick_channel = create_tick_channel(TICK_DELAY, event_tx);

        tick_channel.tick().await?;

        let event = event_rx.recv().await.unwrap();
        assert_eq!(event, Event::Tick);

        Ok(())
    }

    #[tokio::test]
    async fn test_tick_channel_timing() -> Result<()> {
        let (event_tx, mut event_rx) = unbounded_channel();
        let mut tick_channel = create_tick_channel(TICK_DELAY, event_tx);

        let start = std::time::Instant::now();
        tick_channel.tick().await?;
        let _ = event_rx.recv().await.unwrap();
        let end = std::time::Instant::now();

        let total_runtime = (end - start).as_secs_f64();
        let expected_runtime = 1.0 / TICK_DELAY;
        assert_eq!(
            total_runtime <= expected_runtime,
            true,
            "Tick Event took longer to send than expected: {total_runtime}ms"
        );

        Ok(())
    }
}
