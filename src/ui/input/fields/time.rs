use crate::action::Action;
use crate::ui::input::fields::{InputField, IntInputField};
use crate::ui::Component;
use crossterm::event::KeyEvent;
use delegate::delegate;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::Frame;

pub struct TimeInputField {
    hours: IntInputField,
    minutes: IntInputField,
    title: String,
    border_style: Style,
}

impl InputField for TimeInputField {
    fn get_value(&self) -> String {
        format!("{}:{}", self.hours.get_value(), self.minutes.get_value())
    }
    fn border_style(&mut self, style: Style) {
        self.border_style = style;
    }
}
#[allow(dead_code)]
impl TimeInputField {
    pub fn new(title: String) -> Self {
        let hours = IntInputField::new("Hours".into(), 23, None);
        let minutes = IntInputField::new("Minutes".into(), 59, None);
        Self {
            hours,
            minutes,
            border_style: Style::default(),
            title,
        }
    }
}

impl Component for TimeInputField {
    delegate! {
        to if self.hours.get_value().len() != 2 { &mut self.hours } else { &mut self.minutes } {
            fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>>;
        }
    }
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        let hours_len = self.hours.get_value().len();
        let minutes_len = self.minutes.get_value().len();
        if minutes_len == 0 {
            self.hours.handle_key_event(key)?;
        }
        if hours_len == 2 {
            self.minutes.handle_key_event(key)?;
        }
        Ok(None)
    }
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let _ = self.title; // to appease clippy
        let layout =
            Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]).split(area);
        self.hours.border_style(self.border_style);
        self.minutes.border_style(self.border_style);
        self.hours.draw(frame, layout[0])?;
        self.minutes.draw(frame, layout[1])?;
        Ok(())
    }
}
