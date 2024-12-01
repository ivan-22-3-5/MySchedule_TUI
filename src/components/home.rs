use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::components::Schedule;
use crate::{action::Action, config::Config, models};

pub struct Home {
    schedule: Schedule,
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Home {
    pub fn new(schedule: models::Schedule) -> Self {
        Self {
            schedule: Schedule::new(schedule),
            command_tx: None,
            config: Config::default(),
        }
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

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        self.schedule.handle_key_event(key)?;
        Ok(None)
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
        self.schedule.draw(frame, area)?;
        Ok(())
    }
}
