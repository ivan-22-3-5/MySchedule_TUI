use crate::action::Action;
use crate::ui::input::fields::{BorderStyle, InputField};
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use delegate::delegate;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

pub struct InputHandler {
    text: Vec<char>,
    cursor: usize,
    max_length: usize,
}

#[allow(dead_code)]
impl InputHandler {
    pub fn new(initial_text: Option<String>, max_length: usize) -> Self {
        let initial_text: Vec<char> = initial_text
            .unwrap_or_default()
            .chars()
            .take(max_length)
            .collect();
        Self {
            max_length,
            cursor: initial_text.len(),
            text: initial_text,
        }
    }

    pub fn value(&self) -> String {
        self.text.iter().collect()
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    fn try_move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor = self.cursor.saturating_sub(1);
        }
    }

    fn try_move_cursor_right(&mut self) {
        if self.cursor < self.text.len() {
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

    pub fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Right => self.try_move_cursor_right(),
            KeyCode::Left => self.try_move_cursor_left(),
            KeyCode::Char(c) => self.type_char(c),
            KeyCode::Backspace => self.backspace(),
            _ => (),
        };
        Ok(None)
    }
}

pub struct StrInputField {
    title: String,
    is_cursor_visible: bool,
    border_style: Option<BorderStyle>,
    input_handler: InputHandler,
}

impl InputField for StrInputField {
    fn get_value(&self) -> String {
        self.input_handler.value()
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
        Self {
            title: title.unwrap_or_default(),
            border_style: Some((Borders::ALL, Style::default())),
            is_cursor_visible: false,
            input_handler: InputHandler::new(initial_text, max_length),
        }
    }
}

impl Component for StrInputField {
    delegate! {
        to self.input_handler {
            fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>>;
        }
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
            frame.set_cursor_position((
                area.x + self.input_handler.cursor_position() as u16,
                area.y,
            ));
        }
        frame.render_widget(Line::from(self.input_handler.value()), area);
        Ok(())
    }
}
