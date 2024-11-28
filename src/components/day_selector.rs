use super::Component;
use crate::action::Action;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::Frame;

pub struct DaySelector {
    cur_day: u8,
}

impl DaySelector {
    pub fn new() -> Self {
        Self { cur_day: 0 }
    }
}

impl Component for DaySelector {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Left => self.cur_day = (self.cur_day + 6) % 7,
            KeyCode::Right => self.cur_day = (self.cur_day + 1) % 7,
            _ => (),
        }
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
            .highlight_style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .select(self.cur_day as usize);

        frame.render_widget(tabs, area);
        Ok(())
    }
}
