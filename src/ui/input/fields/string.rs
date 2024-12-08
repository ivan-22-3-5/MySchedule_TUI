use crate::action::Action;
use crate::ui::input::fields::{BorderStyle, InputField};
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Position, Rect};
use ratatui::prelude::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

pub struct StrInputField {
    title: String,
    text: Vec<char>,
    cursor: usize,
    max_length: usize,
    is_cursor_visible: bool,
    border_style: Option<BorderStyle>,
}

impl InputField for StrInputField {
    fn get_value(&self) -> String {
        self.text.iter().collect()
    }

    fn borders(&mut self, border_style: Option<BorderStyle>) {
        self.border_style = border_style;
    }
    fn set_cursor_visibility(&mut self, visible: bool) {
        self.is_cursor_visible = visible
    }
}

#[allow(dead_code)]
impl StrInputField {
    pub fn new(title: Option<String>, max_length: usize, initial_text: Option<String>) -> Self {
        let initial_text: Vec<char> = initial_text
            .unwrap_or_default()
            .chars()
            .take(max_length)
            .collect();
        Self {
            title: title.unwrap_or_default(),
            max_length,
            border_style: Some((Borders::ALL, Style::default())),
            cursor: initial_text.len(),
            text: initial_text,
            is_cursor_visible: false,
        }
    }

    fn try_move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor = self.cursor.saturating_sub(1);
        }
    }

    fn try_move_cursor_right(&mut self) {
        if self.cursor < self.text.len() - 1 {
            self.cursor = self.cursor.saturating_add(1);
        }
    }

    fn type_char(&mut self, c: char) {
        if self.text.len() < self.max_length {
            self.text.insert(self.cursor, c);
            self.cursor = self.cursor.saturating_add(1);
        }
    }

    fn backspace(&mut self) {
        if self.cursor > 0 {
            self.text.remove(self.cursor - 1);
            self.cursor = self.cursor.saturating_sub(1);
        }
    }
}

impl Component for StrInputField {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Right => self.try_move_cursor_right(),
            KeyCode::Left => self.try_move_cursor_left(),
            KeyCode::Char(c) => self.type_char(c),
            KeyCode::Backspace => self.backspace(),
            _ => (),
        };
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let mut area = area;
        if let Some(bs) = self.border_style {
            let block = Block::default()
                .borders(bs.0)
                .border_style(bs.1)
                .title(self.title.clone());
            frame.render_widget(block.clone(), area);
            area = block.inner(area);
        }

        if self.is_cursor_visible {
            frame.set_cursor_position(Position::new(area.x + self.cursor as u16, area.y));
        }
        frame.render_widget(Line::from(self.text.iter().collect::<String>()), area);

        Ok(())
    }
}
