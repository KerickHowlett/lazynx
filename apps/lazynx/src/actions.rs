use crossterm::event::Event as CrosstermEvent;
use tokio::sync::mpsc::{
    error::TryRecvError, unbounded_channel, UnboundedReceiver, UnboundedSender,
};

use action::Action;
use events::Event;

struct AppActions {
    should_quit: bool,
    tx: UnboundedSender<Option<Action>>,
}

impl AppActions {
    fn new(tx: UnboundedSender<Option<Action>>) -> Self {
        Self {
            should_quit: false,
            tx,
        }
    }

    fn event_handler(self, event: Event) {
        let action = match event {
            Event::Crossterm(CrosstermEvent::Resize(x, y)) => Some(Action::Resize { x, y }),
            Event::Error(s) => Some(Action::Error(s)),
            Event::Init => Some(Action::Init),
            Event::Quit => Some(Action::Quit),
            Event::Render => Some(Action::Render),
            Event::Tick => Some(Action::Tick),
            _ => None,
        };

        let _ = self.tx.send(action);
    }
}
