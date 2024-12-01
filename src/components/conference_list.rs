use super::Component;
use crate::action::Action;
use crate::models::Conference;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::Frame;

pub struct ConferenceList {
    conferences: Vec<Conference>,
    selected_conference: u8,
}

impl ConferenceList {
    pub fn new(conferences: Vec<Conference>) -> Self {
        Self {
            conferences,
            selected_conference: 0,
        }
    }
}

impl Component for ConferenceList {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        let len = self.conferences.len() as u8;
        self.selected_conference = match key.code {
            KeyCode::Up => (self.selected_conference + (len - 1)) % len,
            KeyCode::Down => (self.selected_conference + 1) % len,
            _ => self.selected_conference,
        };
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let titles = self.conferences.iter().map(|c| c.title.clone());
        let items = titles.map(|title| ListItem::new(title));
        let list = List::new(items).highlight_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );
        let mut state =
            ListState::default().with_selected(Option::from(self.selected_conference as usize));
        frame.render_stateful_widget(list, area, &mut state);
        Ok(())
    }
}
