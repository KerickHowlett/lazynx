use std::rc::Rc;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Status {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Status {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_block(&self) -> Block {
        return Block::default()
            .title(" Status ")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2));
    }

    fn create_layout(&self, area: Rect) -> Rc<[Rect]> {
        return Layout::default()
            .direction(Direction::Vertical)
            .constraints(Constraint::from_lengths([10, 5]))
            .split(area);
    }

    fn get_copyright(&self) -> Paragraph {
        let current_year = chrono::Datelike::year(&chrono::Local::now());
        let copyright = Span::from(format!(
            "Copyright {} {} Kerick Howlett",
            String::from('\u{00A9}'),
            current_year
        ));
        return Paragraph::new(copyright);
    }

    fn get_header(&self) -> Paragraph {
        // NOTE: Don't change the whitespace or alignment for ASCII art text.
        //       Any changes to them will be reflected in the app itself.
        let lazynx_title = String::from(
            r#"
 _                     _   _
| |                   | \ | |
| |     __ _ _____   _|  \| |_  __
| |    / _` |_  / | | | . ` \ \/ /
| |___| (_| |/ /| |_| | |\  |>  <
\_____/\__,_/___|\__, \_| \_/_/\_\
                  __/ |
                 |___ /
"#,
        );

        return Paragraph::new(lazynx_title);
    }
}

impl Component for Status {
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        if let KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } = key
        {
            return Some(Action::Quit);
        };
        None
    }

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
            Action::Init => self.init()?,
            _ => {}
        }
        Ok(None)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = self.create_block();
        let chunks = self.create_layout(block.inner(area));
        frame.render_widget(block, area);

        let header = self.get_header();
        frame.render_widget(header, chunks[0]);

        let copyright = self.get_copyright();
        frame.render_widget(copyright, chunks[1]);
    }
}
