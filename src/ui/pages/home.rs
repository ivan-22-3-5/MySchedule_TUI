use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Mode;
use crate::entities::{Schedule, Settings};
use crate::ui::components::FpsCounter;
use crate::ui::pages::{SchedulePage, SettingsPage};
use crate::ui::Component;
use crate::{action::Action, config::Config};

pub struct Home {
    schedule: SchedulePage,
    settings: SettingsPage,
    fps: FpsCounter,
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
    pub fn new(schedule: Rc<RefCell<Schedule>>, settings: Rc<RefCell<Settings>>) -> Self {
        Self {
            schedule: SchedulePage::new(schedule),
            settings: SettingsPage::new(settings),
            fps: FpsCounter::default(),
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
            ActivePage::Settings => Ok(self.settings.handle_key_event(key)?),
            ActivePage::Schedule => Ok(self.schedule.handle_key_event(key)?),
        }
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        self.fps.update(action.clone())?;
        self.schedule.update(action.clone())?;
        self.settings.update(action.clone())?;
        if let Action::ChangeMode(mode) = action {
            match mode {
                Mode::Settings => {
                    self.active_page = ActivePage::Settings;
                }
                Mode::Schedule => {
                    self.active_page = ActivePage::Schedule;
                }
                _ => {}
            }
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        match self.active_page {
            ActivePage::Settings => {
                self.settings.draw(frame, area)?;
                self.fps.draw(frame, area)?;
            }
            ActivePage::Schedule => {
                self.schedule.draw(frame, area)?;
                self.fps.draw(frame, area)?;
            }
        }
        Ok(())
    }
}
