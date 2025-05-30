use crate::action::Action;
use crate::entities::Time;
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
    border_style: BorderStyle,
    is_cursor_visible: bool,
}

impl InputField for TimeInputField {
    fn get_value(&self) -> String {
        let (hours, minutes) = self.get_parsed_input();
        format!("{:02}:{:02}", hours, minutes)
    }
    fn borders(&mut self, border_style: BorderStyle) {
        self.border_style = border_style;
    }
    fn set_cursor_visibility(&mut self, visible: bool) {
        self.is_cursor_visible = visible
    }
}

impl TimeInputField {
    pub fn new(title: Option<String>, initial_time: Option<Time>) -> Self {
        Self {
            hours: IntInputHandler::new(initial_time.as_ref().map(|time| time.hours() as u32), 23),
            minutes: IntInputHandler::new(
                initial_time.as_ref().map(|time| time.minutes() as u32),
                59,
            ),
            border_style: (Borders::ALL, Style::default()),
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
        let mut width = area.width;
        width -= (width + 1) % 2;

        let area = Layout::horizontal([Constraint::Length(width)]).split(area)[0];
        let area = Layout::vertical([Constraint::Length(3)]).split(area)[0];

        let block = Block::default()
            .title(self.title.clone())
            .borders(self.border_style.0)
            .border_style(self.border_style.1);
        frame.render_widget(block.clone(), area);

        let [_, hours_area, colon_area, minutes_area, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .areas(block.inner(area));

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
