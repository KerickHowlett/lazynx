use color_eyre::eyre::Result;
use tokio::sync::mpsc::UnboundedSender;

use common::{Action, Event};

pub struct RunnerActionPublisher {
    pub action_tx: UnboundedSender<Action>,
}

impl RunnerActionPublisher {
    pub fn new(action_tx: UnboundedSender<Action>) -> Self {
        return Self { action_tx };
    }

    pub async fn send(
        &mut self,
        event: Event,
        other_event_handler: Option<fn(Event) -> Option<Action>>,
    ) -> Result<()> {
        match event {
            Event::Init => self.action_tx.send(Action::Init)?,
            Event::Quit => self.action_tx.send(Action::Quit)?,
            Event::Render => self.action_tx.send(Action::Render)?,
            Event::Tick => self.action_tx.send(Action::Tick)?,
            Event::Resize(x, y) => self.action_tx.send(Action::Resize { x, y })?,
            other_event => match other_event_handler {
                Some(handler) => {
                    if let Some(action) = handler(other_event.clone()) {
                        self.action_tx.send(action)?
                    }
                }
                None => {}
            },
        }

        Ok(())
    }
}

#[cfg(test)]
mod runner_action_publisher_tests {
    use super::RunnerActionPublisher;

    use color_eyre::eyre::Result;
    use test_case::test_case;
    use tokio::sync::mpsc::unbounded_channel;

    use common::{Action, Event};

    #[test_case(Event::Init, Action::Init; "Init")]
    #[test_case(Event::Quit, Action::Quit; "Quit")]
    #[test_case(Event::Render, Action::Render; "Render")]
    #[test_case(Event::Tick, Action::Tick; "Tick")]
    #[test_case(Event::Resize(1, 2), Action::Resize { x: 1, y: 2 }; "Resize")]
    #[tokio::test]
    async fn test_handle_event(event: Event, expected_action: Action) -> Result<()> {
        let (action_tx, mut action_rx) = unbounded_channel();
        let mut publisher = RunnerActionPublisher::new(action_tx);

        publisher.send(event, None).await?;

        let sent_action = action_rx.recv().await;
        assert_eq!(
            sent_action,
            Some(expected_action.clone()),
            "{expected_action:?} was not sent. ${sent_action:?} was received instead.",
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_handle_event_with_added_event_handler() -> Result<()> {
        let other_event_handler = |event: Event| -> Option<Action> {
            match event {
                Event::Closed => Some(Action::Quit),
                _ => None,
            }
        };

        let (action_tx, mut action_rx) = unbounded_channel();
        let mut publisher = RunnerActionPublisher::new(action_tx);

        publisher
            .send(Event::Closed, Some(other_event_handler))
            .await?;

        let sent_action = action_rx.recv().await;
        assert_eq!(sent_action, Some(Action::Quit));

        Ok(())
    }
}
