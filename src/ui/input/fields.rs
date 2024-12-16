mod int;
mod string;
mod time;

use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use delegate::delegate;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Line;
use ratatui::Frame;

pub use string::StrInputField;
pub use time::TimeInputField;

use crate::action::Action;
use ratatui::prelude::Style;
use ratatui::widgets::{Block, Borders};

type BorderStyle = (Borders, Style);
#[allow(dead_code)]
pub trait InputField: Component {
    fn get_value(&self) -> String;
    fn borders(&mut self, border_style: BorderStyle);
    fn set_cursor_visibility(&mut self, visible: bool);
}

pub struct BaseInputField {
    title: String,
    is_cursor_visible: bool,
    border_style: BorderStyle,
    input_handler: Box<dyn InputHandler>,
    left_padding: u16,
}

impl InputField for BaseInputField {
    fn get_value(&self) -> String {
        self.input_handler.value()
    }

    fn borders(&mut self, border_style: BorderStyle) {
        self.border_style = border_style;
    }

    fn set_cursor_visibility(&mut self, visible: bool) {
        self.is_cursor_visible = visible
    }
}

#[allow(dead_code)]
impl BaseInputField {
    pub fn new(title: Option<String>, input_handler: Box<dyn InputHandler>) -> Self {
        Self {
            title: title.unwrap_or_default(),
            border_style: (Borders::ALL, Style::default()),
            is_cursor_visible: false,
            input_handler,
            left_padding: 0,
        }
    }

    fn recalculate_padding(&mut self, width: u16) {
        let text_len = self.input_handler.len() as u16;
        let cursor_pos = self.input_handler.cursor_position() as u16;

        if !(self.left_padding..(self.left_padding + width)).contains(&cursor_pos) {
            self.left_padding = self
                .left_padding
                .max(cursor_pos.saturating_sub(width)) // if cursor is out of right bound set padding to cursor_pos - width
                .min(cursor_pos); // if cursor is out of left bound set padding to cursor_pos
        }
        self.left_padding = self.left_padding.min(text_len.saturating_sub(width));
        // move padding to the left if there is empty space that is not occupied by the text
    }
}

impl Component for BaseInputField {
    delegate! {
        to self.input_handler {
            fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>>;
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let area = Layout::vertical([Constraint::Length(3)]).split(area)[0];

        self.recalculate_padding(area.width - 1);

        if self.is_cursor_visible {
            frame.set_cursor_position((
                area.x + self.input_handler.cursor_position() as u16 - self.left_padding,
                area.y + 1,
            ));
        }

        let string_slice_to_show: String = self
            .input_handler
            .value()
            .chars()
            .skip(self.left_padding as usize)
            .take(area.width as usize)
            .collect();

        let block = Block::default()
            .borders(self.border_style.0)
            .border_style(self.border_style.1)
            .title(self.title.clone());
        frame.render_widget(block.clone(), area);
        frame.render_widget(Line::from(string_slice_to_show), block.inner(area));
        Ok(())
    }
}

type ValidateFn = Box<dyn Fn(&str) -> bool>;

pub trait InputHandler {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>>;
    fn value(&self) -> String;
    fn cursor_position(&self) -> usize;
    fn len(&self) -> usize;
}

pub struct BaseInputHandler {
    text: Vec<char>,
    cursor: usize,
    max_length: usize,
    validate: ValidateFn,
}

impl InputHandler for BaseInputHandler {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Right => self.try_move_cursor_right(),
            KeyCode::Left => self.try_move_cursor_left(),
            KeyCode::Char(c) => self.try_insert_char(c),
            KeyCode::Backspace => self.backspace(),
            _ => (),
        };
        Ok(None)
    }

    fn value(&self) -> String {
        self.text.iter().collect()
    }

    fn cursor_position(&self) -> usize {
        self.cursor
    }

    fn len(&self) -> usize {
        self.text.len()
    }
}

#[allow(dead_code)]
impl BaseInputHandler {
    pub fn new(initial: Option<String>, max_length: usize, validate: Option<ValidateFn>) -> Self {
        let initial_text: Vec<char> = initial
            .unwrap_or_default()
            .chars()
            .take(max_length)
            .collect();
        Self {
            max_length,
            cursor: initial_text.len(),
            text: initial_text,
            validate: validate.unwrap_or(Box::new(|_| true)),
        }
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

    fn try_insert_char(&mut self, c: char) {
        if self.text.len() < self.max_length {
            let mut new_value = self.text.clone();
            new_value.insert(self.cursor, c);
            if (self.validate)(&new_value.iter().collect::<String>()) {
                self.text = new_value;
                self.cursor += 1;
            }
        }
    }

    fn backspace(&mut self) {
        if self.cursor > 0 {
            self.text.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }
}
