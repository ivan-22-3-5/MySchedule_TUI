use crate::action::Action;
use crate::models::Conference;
use crate::theme::THEME;
use crate::ui::components::Selector;
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::Frame;
use std::sync::Arc;

pub struct ConferenceList {
    conferences: Arc<Vec<Conference>>,
    selector: Selector,
}

impl ConferenceList {
    pub fn new(conferences: Arc<Vec<Conference>>) -> Self {
        Self {
            selector: Selector::new(conferences.len() as u64),
            conferences,
        }
    }
}

impl Component for ConferenceList {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Up => self.selector.prev(),
            KeyCode::Down => self.selector.next(),
            KeyCode::Enter => {
                if let Some(c) = self.conferences.get(self.selector.index as usize) {
                    c.open();
                }
            }
            _ => (),
        };
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let titles = self.conferences.iter().map(|c| c.title.clone());
        let items = titles.map(ListItem::new);
        let list = List::new(items).highlight_style(THEME.selected);
        let mut state =
            ListState::default().with_selected(Option::from(self.selector.index as usize));
        frame.render_stateful_widget(list, area, &mut state);
        Ok(())
    }
}
