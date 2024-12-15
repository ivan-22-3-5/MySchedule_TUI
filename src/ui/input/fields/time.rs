use crate::action::Action;
use crate::theme::THEME;
use crate::ui::input::fields::int::IntInputHandler;
use crate::ui::input::fields::{BorderStyle, InputField, InputHandler};
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Style;
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;
use std::str::FromStr;

#[derive(Clone)]
pub struct TimeStr(String);

impl TimeStr {
    pub fn parse(time: &str) -> Result<TimeStr, &'static str> {
        const MAX_HOURS: u8 = 23;
        const MAX_MINUTES: u8 = 59;

        fn validate_hours(hours: u8) -> Result<(), &'static str> {
            if hours <= MAX_HOURS {
                Ok(())
            } else {
                Err("Invalid hour: must be between 0 and 23")
            }
        }

        fn validate_minutes(minutes: u8) -> Result<(), &'static str> {
            if minutes <= MAX_MINUTES {
                Ok(())
            } else {
                Err("Invalid minute: must be between 0 and 59")
            }
        }

        let mut parts = time.split(':').map(|s| s.parse::<u8>());
        match (parts.next(), parts.next()) {
            (Some(Ok(hours)), Some(Ok(minutes))) => {
                validate_hours(hours)?;
                validate_minutes(minutes)?;
                Ok(Self(time.to_string()))
            }
            _ => Err("Invalid time format: must be HH:MM"),
        }
    }

    pub fn hours(&self) -> u32 {
        self.0
            .split(':')
            .next()
            .unwrap()
            .parse()
            .expect("TimeStr should always have valid hours")
    }

    pub fn minutes(&self) -> u32 {
        self.0
            .split(':')
            .nth(1)
            .unwrap()
            .parse()
            .expect("TimeStr should always have valid minutes")
    }
}

impl FromStr for TimeStr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TimeStr::parse(s)
    }
}

#[derive(Default)]
enum SelectedField {
    #[default]
    Hours,
    Minutes,
}

pub struct TimeInputField {
    hours: IntInputHandler,
    minutes: IntInputHandler,
    title: String,
    selected_field: SelectedField,
    border_style: Option<BorderStyle>,
    is_cursor_visible: bool,
}

impl InputField for TimeInputField {
    fn get_value(&self) -> String {
        let (hours, minutes) = self.get_parsed_input();
        format!("{:02}:{:02}", hours, minutes)
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
    pub fn new(title: Option<String>, initial_time: Option<TimeStr>) -> Self {
        Self {
            hours: IntInputHandler::new(initial_time.clone().map(|time| time.hours()), 23),
            minutes: IntInputHandler::new(initial_time.map(|time| time.minutes()), 59),
            border_style: Some((Borders::ALL, Style::default())),
            is_cursor_visible: false,
            selected_field: SelectedField::default(),
            title: title.unwrap_or_default(),
        }
    }

    fn get_parsed_input(&self) -> (u8, u8) {
        let hours: u8 = self
            .hours
            .value()
            .parse()
            .expect("IntInputHandler should always give valid number");
        let minutes: u8 = self
            .minutes
            .value()
            .parse()
            .expect("IntInputHandler should always give valid number");
        (hours, minutes)
    }
}

impl Component for TimeInputField {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        if let KeyCode::Left | KeyCode::Right = key.code {
            self.selected_field = match self.selected_field {
                SelectedField::Hours => SelectedField::Minutes,
                SelectedField::Minutes => SelectedField::Hours,
            };
        } else {
            match self.selected_field {
                SelectedField::Hours => self.hours.handle_key_event(key)?,
                SelectedField::Minutes => self.minutes.handle_key_event(key)?,
            };
        }
        Ok(None)
    }
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let mut width = self.title.len() + 2;
        width += std::cmp::max((width + 1) % 2, 9);
        let area = Layout::horizontal([Constraint::Length(width as u16)]).split(area)[0];
        let mut area = Layout::vertical([Constraint::Length(3)]).split(area)[0];
        if let Some(bs) = self.border_style {
            let block = Block::default()
                .title(self.title.clone())
                .borders(bs.0)
                .border_style(bs.1);
            frame.render_widget(block.clone(), area);
            area = block.inner(area);
        }
        let [_, hours_area, colon_area, minutes_area, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .areas(area);

        let (hours, minutes) = self.get_parsed_input();
        let mut minutes = Span::raw(format!("{:02}", minutes));
        let mut hours = Span::raw(format!("{:02}", hours));
        if self.is_cursor_visible {
            match self.selected_field {
                SelectedField::Hours => hours.style = THEME.selected_text,
                SelectedField::Minutes => minutes.style = THEME.selected_text,
            };
        }

        frame.render_widget(hours, hours_area);
        frame.render_widget(minutes, minutes_area);
        frame.render_widget(Span::raw(":"), colon_area);
        Ok(())
    }
}
