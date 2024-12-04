use super::{Component, Selector};
use crate::action::Action;
use crate::models;
use crate::theme::THEME;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::Frame;
use std::sync::Arc;

pub struct Settings {
    selector: Selector,
    settings: Arc<models::Settings>,
}

impl Settings {
    pub fn new(settings: Arc<models::Settings>) -> Self {
        Self {
            selector: Selector::new(3),
            settings,
        }
    }
}

impl Component for Settings {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Up => self.selector.prev(),
            KeyCode::Down => self.selector.next(),
            _ => (),
        };
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let _ = self.settings; // to appease clippy
        let items = ["1", "2", "3"].map(ListItem::new);
        let list = List::new(items).highlight_style(THEME.selected);
        let mut state =
            ListState::default().with_selected(Option::from(self.selector.index as usize));
        frame.render_stateful_widget(list, area, &mut state);
        Ok(())
    }
}
