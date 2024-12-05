use crate::action::Action;
use crate::models;
use crate::ui::components::{ConferenceList, DaySelector};
use crate::ui::Component;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;
use std::sync::Arc;

pub struct SchedulePage {
    selector: DaySelector,
    days: [ConferenceList; 7],
}

impl SchedulePage {
    pub fn new(schedule: Arc<models::Schedule>) -> Self {
        Self {
            selector: DaySelector::new(),
            days: schedule.clone_into_array().map(ConferenceList::new),
        }
    }
}

impl Component for SchedulePage {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        self.selector.handle_key_event(key)?;

        self.days
            .get_mut(self.selector.selected_day() as usize)
            .unwrap()
            .handle_key_event(key)?;
        Ok(None)
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        self.selector.update(action.clone())?;
        self.days
            .get_mut(self.selector.selected_day() as usize)
            .unwrap()
            .update(action)?;
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let layout: [Rect; 2] =
            Layout::vertical([Constraint::Percentage(15), Constraint::Percentage(85)]).areas(area);
        self.selector.draw(frame, layout[0])?;
        self.days[self.selector.selected_day() as usize].draw(frame, layout[1])?;
        Ok(())
    }
}
