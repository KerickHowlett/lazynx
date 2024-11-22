use color_eyre::eyre::Result;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::{
    action::Action,
    components::{status::Status, Component},
    config::Config,
    tui,
};

#[derive(Default)]
pub struct Runner {
    pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
}

impl Runner {
    pub fn new(config: Config, tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let status = Status::new();
        Ok(Self {
            components: vec![Box::new(status)],
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
        action_rx: &mut tokio::sync::mpsc::UnboundedReceiver<Action>,
        tui: &mut tui::Tui,
        action_tx: &tokio::sync::mpsc::UnboundedSender<Action>,
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
        event: tui::Event,
        action_tx: &UnboundedSender<Action>,
    ) -> Result<()> {
        match event {
            tui::Event::Init => action_tx.send(Action::Init)?,
            tui::Event::Quit => action_tx.send(Action::Quit)?,
            tui::Event::Render => action_tx.send(Action::Render)?,
            tui::Event::Tick => action_tx.send(Action::Tick)?,
            tui::Event::Resize(x, y) => action_tx.send(Action::Resize { x, y })?,
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
