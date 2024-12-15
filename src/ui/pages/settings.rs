use crate::action::Action;
use crate::models::Settings;
use crate::theme::THEME;
use crate::ui::components::Selector;
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::Frame;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SettingsPage {
    selector: Selector,
    settings: Rc<RefCell<Settings>>,
}

impl SettingsPage {
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        Self {
            selector: Selector::new(3),
            settings,
        }
    }
}

impl Component for SettingsPage {
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
        let list = List::new(items).highlight_style(THEME.selected_text);
        let mut state =
            ListState::default().with_selected(Option::from(self.selector.index as usize));
        frame.render_stateful_widget(list, area, &mut state);
        Ok(())
    }
}
