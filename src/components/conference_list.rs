use super::Component;
use crate::action::Action;
use crate::components::Selector;
use crate::models::Conference;
use crate::theme::THEME;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::Frame;
use std::sync::Arc;

pub struct ConferenceList {
    conferences: Arc<Vec<Conference>>,
    conference_selector: Selector,
}

impl ConferenceList {
    pub fn new(conferences: Arc<Vec<Conference>>) -> Self {
        Self {
            conference_selector: Selector::new(conferences.len() as u64),
            conferences,
        }
    }
}

impl Component for ConferenceList {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Up => self.conference_selector.prev(),
            KeyCode::Down => self.conference_selector.next(),
            KeyCode::Enter => self.conferences[self.conference_selector.index as usize].open(),
            _ => (),
        };
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let titles = self.conferences.iter().map(|c| c.title.clone());
        let items = titles.map(ListItem::new);
        let list = List::new(items).highlight_style(THEME.selected);
        let mut state = ListState::default()
            .with_selected(Option::from(self.conference_selector.index as usize));
        frame.render_stateful_widget(list, area, &mut state);
        Ok(())
    }
}
