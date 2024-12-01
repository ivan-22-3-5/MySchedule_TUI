use super::{Component, ConferenceList, DaySelector};
use crate::action::Action;
use crate::models;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;

pub struct Schedule {
    day_selector: DaySelector,
    days: [ConferenceList; 7],
}

impl Schedule {
    pub fn new(schedule: models::Schedule) -> Self {
        Self {
            day_selector: DaySelector::new(),
            days: schedule.to_array().map(|day| ConferenceList::new(day)),
        }
    }
}

impl Component for Schedule {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        self.day_selector.handle_key_event(key)?;
        self.days[self.day_selector.selected_day as usize].handle_key_event(key)?;
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let layout: [Rect; 2] =
            Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]).areas(area);
        self.day_selector.draw(frame, layout[0])?;
        self.days[self.day_selector.selected_day as usize].draw(frame, layout[1])?;
        Ok(())
    }
}
