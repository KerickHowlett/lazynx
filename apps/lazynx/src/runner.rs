use color_eyre::eyre::Result;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use app_config::Config;
use common::{Action, Component, Event};
use tui::{self};

#[derive(Default)]
pub struct Runner {
    pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component<Config>>>,
    pub should_quit: bool,
    pub should_suspend: bool,
}

impl Runner {
    pub fn new(
        config: Config,
        tick_rate: f64,
        frame_rate: f64,
        components: Vec<Box<dyn Component<Config>>>,
    ) -> Result<Self> {
        Ok(Self {
            components,
            config,
            frame_rate,
            should_quit: false,
            should_suspend: false,
            tick_rate,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = unbounded_channel();

        let mut tui = tui::Tui::new()?
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate)
            .mouse(true)
            .paste(true);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
        }
        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }
        for component in self.components.iter_mut() {
            component.init()?;
        }

        loop {
            if let Some(e) = tui.next().await {
                self.handle_event(e, &action_tx).await?;
            }

            self.handle_action(&mut action_rx, &mut tui, &action_tx)
                .await?;

            if self.should_suspend {
                self.suspend_tui(&mut tui, &action_tx)?;
                continue;
            }

            if self.should_quit {
                tui.stop()?;
                break;
            }
        }

        tui.exit()?;
        Ok(())
    }

    async fn handle_action(
        &mut self,
        action_rx: &mut UnboundedReceiver<Action>,
        tui: &mut tui::Tui,
        action_tx: &UnboundedSender<Action>,
    ) -> Result<()> {
        while let Ok(action) = action_rx.try_recv() {
            if action != Action::Tick && action != Action::Render {
                log::debug!("{action:?}");
            }

            match action {
                Action::Quit => {
                    self.should_quit = true;
                    println!("Bye!");
                }
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::Render => {
                    tui.draw(|f| {
                        for component in self.components.iter_mut() {
                            component.draw(f, f.area());
                        }
                    })?;
                }
                _ => {}
            }

            for component in self.components.iter_mut() {
                if let Some(action) = component.update(action.clone())? {
                    action_tx.send(action)?
                };
            }
        }

        Ok(())
    }

    async fn handle_event(
        &mut self,
        event: Event,
        action_tx: &UnboundedSender<Action>,
    ) -> Result<()> {
        match event {
            Event::Init => action_tx.send(Action::Init)?,
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize { x, y })?,
            other_event => {
                for component in self.components.iter_mut() {
                    if let Some(action) = component.handle_events(other_event.clone()) {
                        action_tx.send(action)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn suspend_tui(
        &mut self,
        tui: &mut tui::Tui,
        action_tx: &UnboundedSender<Action>,
    ) -> Result<()> {
        tui.suspend()?;
        action_tx.send(Action::Resume)?;

        *tui = tui::Tui::new()?
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        tui.enter()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Runner;
    use crate::assert_event_handler;
    use app_config::Config;
    use color_eyre::eyre::Result;
    use common::{Action, Component, Event};
    use pretty_assertions::assert_eq;
    use ratatui::layout::Rect;
    use tokio::sync::mpsc::unbounded_channel;

    fn setup() -> Result<Runner> {
        return Runner::new(Config::default(), 0.0, 0.0, vec![]);
    }

    #[test]
    fn test_runner_instantiation() {
        let runner = setup();
        assert_eq!(
            runner.is_ok(),
            true,
            "Runner did not instantiate: {:?}",
            runner.err()
        );
    }

    #[tokio::test]
    async fn test_handle_event_init() -> Result<()> {
        let mut runner = setup()?;
        let (tx, mut rx) = unbounded_channel();
        runner.handle_event(Event::Init, &tx).await?;
        assert_eq!(Some(Action::Init), rx.recv().await);
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_event_quit() -> Result<()> {
        let mut runner = Runner::new(Config::default(), 0.0, 0.0, vec![])?;
        let (tx, mut rx) = unbounded_channel();
        runner.handle_event(Event::Quit, &tx).await?;
        assert_eq!(Some(Action::Quit), rx.recv().await);
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_event_render() -> Result<()> {
        let mut runner = Runner::new(Config::default(), 0.0, 0.0, vec![])?;
        let (tx, mut rx) = unbounded_channel();
        runner.handle_event(Event::Render, &tx).await?;
        assert_eq!(Some(Action::Render), rx.recv().await);
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_event_tick() -> Result<()> {
        let mut runner = Runner::new(Config::default(), 0.0, 0.0, vec![])?;
        let (tx, mut rx) = unbounded_channel();
        runner.handle_event(Event::Tick, &tx).await?;
        assert_eq!(Some(Action::Tick), rx.recv().await);
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_event_resize() -> Result<()> {
        let mut runner = Runner::new(Config::default(), 0.0, 0.0, vec![])?;
        let (tx, mut rx) = unbounded_channel();
        runner.handle_event(Event::Resize(1, 2), &tx).await?;
        assert_eq!(Some(Action::Resize { x: 1, y: 2 }), rx.recv().await);
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_event_render_2() -> Result<()> {
        assert_event_handler!(Event::Resize(1, 2), Action::Resize { x: 1, y: 2 });
        Ok(())
    }

    #[tokio::test]
    async fn test_any_other_event() -> Result<()> {
        #[derive(Default)]
        struct MockComponent {}

        impl Component<Config> for MockComponent {
            fn handle_events(&mut self, event: Event) -> Option<Action> {
                match event {
                    Event::Closed => Some(Action::Quit),
                    _ => None,
                }
            }

            fn draw(&mut self, _f: &mut tui::Frame, _area: Rect) {}
        }

        let mock = Box::new(MockComponent::default());
        let mut runner = Runner::new(Config::default(), 0.0, 0.0, vec![mock])?;
        let (tx, mut rx) = unbounded_channel();
        runner.handle_event(Event::Closed, &tx).await?;
        assert_eq!(Some(Action::Quit), rx.recv().await);

        Ok(())
    }

    #[macro_export]
    macro_rules! assert_event_handler {
        ($event:expr, $action:expr) => {{
            #[derive(Default)]
            struct MockComponent {}

            impl Component<Config> for MockComponent {
                fn handle_events(&mut self, event: Event) -> Option<Action> {
                    match event {
                        $event => Some($action),
                        _ => None,
                    }
                }

                fn draw(&mut self, _f: &mut tui::Frame, _area: Rect) {}
            }

            let mock = Box::new(MockComponent::default());
            let mut runner = Runner::new(Config::default(), 0.0, 0.0, vec![mock])?;
            let (tx, mut rx) = unbounded_channel();

            runner.handle_event($event, &tx).await?;

            assert_eq!(Some($action), rx.recv().await);
        }};
    }
}
