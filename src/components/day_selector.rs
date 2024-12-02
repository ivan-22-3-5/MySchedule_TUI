use super::Component;
use crate::action::Action;
use crate::components::Selector;
use crate::theme::THEME;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::Frame;

pub struct DaySelector {
    day_selector: Selector,
}

impl DaySelector {
    pub fn new() -> Self {
        Self {
            day_selector: Selector::new(7),
        }
    }

    pub fn selected_day(&self) -> u64 {
        self.day_selector.index
    }
}

impl Component for DaySelector {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Left => self.day_selector.prev(),
            KeyCode::Right => self.day_selector.next(),
            _ => (),
        };
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let titles =
            ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].map(|day| format!("  {}  ", day));

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Schedule"))
            .style(Style::default().fg(Color::White))
            .divider("")
            .padding("", "")
            .highlight_style(THEME.selected)
            .select(self.day_selector.index as usize);

        frame.render_widget(tabs, area);
        Ok(())
    }
}
