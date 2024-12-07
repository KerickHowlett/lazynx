use color_eyre::eyre::Result;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use app_config::Config;
use common::{Action, Component, Event};
use tui::Tui;

pub struct Runner<TApp: Component<Config>> {
    pub action_rx: UnboundedReceiver<Action>,
    pub action_tx: UnboundedSender<Action>,
    pub app: TApp,
    pub config: Config,
    pub frame_rate: f64,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub tick_rate: f64,
    pub tui: Tui,
}

impl<TApp: Component<Config>> Runner<TApp> {
    pub fn new(
        app: TApp,
        config: Config,
        tick_rate: f64,
        frame_rate: f64,
        action_tx: UnboundedSender<Action>,
        action_rx: UnboundedReceiver<Action>,
        tui: Tui,
    ) -> Result<Self> {
        let tui = tui
            .set_tick_rate(tick_rate)
            .set_frame_rate(frame_rate)
            .set_mouse(true)
            .set_paste(true);

        Ok(Self {
            action_rx,
            action_tx,
            app,
            config,
            frame_rate,
            should_quit: false,
            should_suspend: false,
            tick_rate,
            tui,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.tui.enter()?;

        self.app.register_action_handler(self.action_tx.clone())?;
        self.app.register_config_handler(self.config.clone())?;
        self.app.init()?;

        loop {
            if let Some(event) = self.tui.next().await {
                self.handle_event(event).await?;
            }

            self.handle_action().await?;

            if self.should_suspend {
                self.suspend_tui()?;
                continue;
            }

            if self.should_quit {
                self.tui.stop()?;
                break;
            }
        }

        self.tui.exit()?;

        Ok(())
    }

    async fn handle_action(&mut self) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            if action != Action::Tick && action != Action::Render {
                log::debug!("{action:?}");
            }

            match action {
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::Render => {
                    self.tui.draw(|f| {
                        self.app.draw(f, f.area());
                    })?;
                }
                _ => {}
            }

            if let Some(action) = self.app.update(action.clone())? {
                self.action_tx.send(action)?
            }
        }

        Ok(())
    }

    async fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Init => self.action_tx.send(Action::Init)?,
            Event::Quit => self.action_tx.send(Action::Quit)?,
            Event::Render => self.action_tx.send(Action::Render)?,
            Event::Tick => self.action_tx.send(Action::Tick)?,
            Event::Resize(x, y) => self.action_tx.send(Action::Resize { x, y })?,
            other_event => {
                if let Some(action) = self.app.handle_events(other_event.clone()) {
                    self.action_tx.send(action)?
                }
            }
        }

        Ok(())
    }

    fn suspend_tui(&mut self) -> Result<()> {
        self.tui.suspend()?;
        self.action_tx.send(Action::Resume)?;

        self.tui = tui::Tui::new()?
            .set_tick_rate(self.tick_rate)
            .set_frame_rate(self.frame_rate)
            .set_mouse(true)
            .set_paste(true);

        self.tui.enter()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::{path, time::Duration};

    use super::Runner;
    use app_config::Config;
    use color_eyre::eyre::Result;
    use common::{Action, Component, Event};
    use pretty_assertions::assert_eq;
    use ratatui::layout::Rect;
    use test_case::test_case;
    use tokio::{
        sync::mpsc::{unbounded_channel, UnboundedSender},
        time::timeout,
    };

    #[derive(Default)]
    struct MockApp {
        config: Config,
        init_called: bool,
        is_rendered: bool,
        action_handler_tx: Option<UnboundedSender<Action>>,
        updated_with: Option<Action>,
    }

    impl Component<Config> for MockApp {
        fn draw(&mut self, _f: &mut tui::Frame, _area: Rect) {
            self.is_rendered = true;
        }

        fn handle_events(&mut self, event: Event) -> Option<Action> {
            match event {
                Event::Closed => Some(Action::Quit),
                _ => None,
            }
        }

        fn init(&mut self) -> Result<()> {
            self.init_called = true;
            Ok(())
        }

        fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
            self.action_handler_tx = Some(tx.clone());
            Ok(())
        }

        fn register_config_handler(&mut self, config: Config) -> Result<()> {
            self.config = config;
            Ok(())
        }

        fn update(&mut self, action: Action) -> Result<Option<Action>> {
            self.updated_with = Some(action);
            Ok(None)
        }
    }

    const TMP_DIR: &str = "/tmp";

    fn setup() -> Runner<MockApp> {
        let mock_app = MockApp::default();
        let mut mock_config = Config::default();
        mock_config.config.data_dir = path::PathBuf::from(TMP_DIR);

        let tui = tui::Tui::new().unwrap();
        let (action_tx, action_rx) = unbounded_channel();

        return Runner::new(mock_app, mock_config, 4.0, 60.0, action_tx, action_rx, tui).unwrap();
    }

    // @SECTION: Runner Instantiation Test

    #[test]
    fn test_runner_instantiation() {
        let (action_tx, action_rx) = unbounded_channel();

        let runner = Runner::new(
            MockApp::default(),
            Config::default(),
            4.0,
            60.0,
            action_tx,
            action_rx,
            tui::Tui::new().unwrap(),
        );

        assert_eq!(
            runner.is_ok(),
            true,
            "Runner did not instantiate: {:?}",
            runner.err()
        );
    }

    // @SECTION: Runner.handle_event Tests

    #[test_case(Event::Init, Action::Init; "Init")]
    #[test_case(Event::Quit, Action::Quit; "Quit")]
    #[test_case(Event::Render, Action::Render; "Render")]
    #[test_case(Event::Tick, Action::Tick; "Tick")]
    #[test_case(Event::Resize(1, 2), Action::Resize { x: 1, y: 2 }; "Resize")]
    #[test_case(Event::Closed, Action::Quit; "Any Other Event (Established in MockComponent)")]
    #[tokio::test]
    async fn test_handle_event(event: Event, expected_action: Action) -> Result<()> {
        let mut runner = setup();

        runner.handle_event(event).await?;

        let sent_action = runner.action_rx.recv().await;
        assert_eq!(
            sent_action,
            Some(expected_action.clone()),
            "{expected_action:?} was not sent. ${sent_action:?} was received instead.",
        );

        Ok(())
    }

    // @SECTION: Runner.handle_action Tests

    #[tokio::test]
    async fn test_handle_action_quit() -> Result<()> {
        let mut runner = setup();
        runner.should_quit = false;

        runner.action_tx.send(Action::Quit)?;
        runner.handle_action().await?;

        assert_eq!(runner.should_quit, true);

        Ok(())
    }

    #[tokio::test]
    async fn test_handle_action_suspend() -> Result<()> {
        let mut runner = setup();
        runner.should_suspend = false;

        runner.action_tx.send(Action::Suspend)?;
        runner.handle_action().await?;

        assert_eq!(runner.should_suspend, true);

        Ok(())
    }

    #[tokio::test]
    async fn test_handle_action_resume() -> Result<()> {
        let mut runner = setup();
        runner.should_suspend = true;

        runner.action_tx.send(Action::Resume)?;
        runner.handle_action().await?;

        assert_eq!(runner.should_suspend, false);

        Ok(())
    }

    #[tokio::test]
    async fn test_handle_action_tick() -> Result<()> {
        let mut runner = setup();
        runner.app.updated_with = None;

        runner.action_tx.send(Action::Tick)?;
        runner.handle_action().await?;

        assert_eq!(runner.app.updated_with.unwrap(), Action::Tick);

        Ok(())
    }

    #[tokio::test]
    async fn test_handle_action_render() -> Result<()> {
        let mut runner = setup();
        runner.app.is_rendered = false;

        runner.action_tx.send(Action::Render)?;
        runner.handle_action().await?;

        assert_eq!(runner.app.is_rendered, true);

        Ok(())
    }

    // @SECTION: Runner.run Tests

    #[tokio::test]
    async fn test_run_call_app_init() -> Result<()> {
        let mut runner = setup();

        tokio::select! {
            _ = runner.run() => {},
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
            },
        }

        assert_eq!(runner.app.init_called, true);

        Ok(())
    }

    #[tokio::test]
    async fn test_run_register_action_handler() -> Result<()> {
        let mut runner = setup();

        tokio::select! {
            _ = runner.run() => {},
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
            },
        }

        assert_eq!(runner.app.action_handler_tx.is_some(), true);

        Ok(())
    }

    #[tokio::test]
    async fn test_run_register_config() -> Result<()> {
        let mut runner = setup();

        tokio::select! {
            _ = runner.run() => {},
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
            },
        }

        let actual_data_dir = runner.app.config.config.data_dir.to_str().unwrap();
        assert_eq!(actual_data_dir, TMP_DIR);

        Ok(())
    }

    #[tokio::test]
    async fn test_run_should_exit_tui_on_quit() -> Result<()> {
        let mut runner = setup();
        runner.should_quit = true;

        if let Err(error) = timeout(Duration::from_secs(2), runner.run()).await {
            panic!("Runner.run() failed to exit tui: {error:?}");
        }

        Ok(())
    }
}
