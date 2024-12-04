use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::components::{Schedule, Settings};
use crate::{action::Action, config::Config, models};

pub struct Home {
    schedule: Schedule,
    settings: Settings,
    active_page: ActivePage,
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

#[derive(Default)]
enum ActivePage {
    #[default]
    Schedule,
    Settings,
}

impl Home {
    pub fn new(schedule: Arc<models::Schedule>, settings: Arc<models::Settings>) -> Self {
        Self {
            schedule: Schedule::new(schedule),
            settings: Settings::new(settings),
            active_page: ActivePage::default(),
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
        match self.active_page {
            ActivePage::Settings => self.settings.handle_key_event(key)?,
            ActivePage::Schedule => self.schedule.handle_key_event(key)?,
        };
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
            Action::Settings => self.active_page = ActivePage::Settings,
            Action::Schedule => self.active_page = ActivePage::Schedule,
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        match self.active_page {
            ActivePage::Settings => self.settings.draw(frame, area)?,
            ActivePage::Schedule => self.schedule.draw(frame, area)?,
        }
        Ok(())
    }
}
