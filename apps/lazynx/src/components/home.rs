use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

const LAZYNX_TITLE: &str = r#"
 _                     _   _
| |                   | \ | |
| |     __ _ _____   _|  \| |_  __
| |    / _` |_  / | | | . ` \ \/ /
| |___| (_| |/ /| |_| | |\  |>  <
\_____/\__,_/___|\__, \_| \_/_/\_\
                  __/ |
                 |___ /
"#;

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let block = Block::default()
            .title_top("Status")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(Constraint::from_lengths([10, 5]))
            .split(block.inner(area));
        frame.render_widget(block, area);

        let header = Paragraph::new(LAZYNX_TITLE);
        frame.render_widget(header, chunks[0]);

        let current_year = chrono::Datelike::year(&chrono::Local::now());
        let copyright = Span::from(format!(
            "Copyright {} {} Kerick Howlett",
            String::from('\u{00A9}'),
            current_year
        ));
        frame.render_widget(copyright, chunks[1]);

        Ok(())
    }
}
