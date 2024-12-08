use crate::action::Action;
use crate::ui::input::fields::{BorderStyle, InputField, IntInputField};
use crate::ui::Component;
use crossterm::event::KeyEvent;
use delegate::delegate;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Style;
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

pub struct TimeInputField {
    hours: IntInputField,
    minutes: IntInputField,
    title: String,
    border_style: Option<BorderStyle>,
    is_cursor_visible: bool,
}

impl InputField for TimeInputField {
    fn get_value(&self) -> String {
        format!("{}:{}", self.hours.get_value(), self.minutes.get_value())
    }
    fn borders(&mut self, border_style: Option<BorderStyle>) {
        self.border_style = border_style;
    }
    fn set_cursor_visibility(&mut self, visible: bool) {
        self.is_cursor_visible = visible
    }
}
#[allow(dead_code)]
impl TimeInputField {
    pub fn new(title: Option<String>) -> Self {
        let mut hours = IntInputField::new(None, 23, None);
        let mut minutes = IntInputField::new(None, 59, None);
        hours.borders(None);
        minutes.borders(None);
        Self {
            hours,
            minutes,
            border_style: Some((Borders::ALL, Style::default())),
            is_cursor_visible: false,
            title: title.unwrap_or_default(),
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
        let mut area = area;
        if let Some(bs) = self.border_style {
            let block = Block::default()
                .title(self.title.clone())
                .borders(bs.0)
                .border_style(bs.1);
            frame.render_widget(block.clone(), area);
            area = block.inner(area);
        }

        let layout =
            Layout::horizontal([Constraint::Max(2), Constraint::Max(1), Constraint::Max(2)])
                .split(area);
        self.minutes.set_cursor_visibility(false);
        self.hours.set_cursor_visibility(false);
        if self.is_cursor_visible {
            if self.hours.get_value().len() != 2 {
                self.hours.set_cursor_visibility(true);
            } else {
                self.minutes.set_cursor_visibility(true);
            }
        }
        self.hours.draw(frame, layout[0])?;
        frame.render_widget(Span::raw(":"), layout[1]);
        self.minutes.draw(frame, layout[2])?;
        Ok(())
    }
}
