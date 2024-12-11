use crate::action::Action;
use crossterm::event::{KeyCode, KeyEvent};
use delegate::delegate;

type ValidateFn = Box<dyn Fn(&str) -> bool>;

pub struct InputHandler {
    text: Vec<char>,
    cursor: usize,
    max_length: usize,
    validate: ValidateFn,
}

#[allow(dead_code)]
impl InputHandler {
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

    pub fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Right => self.try_move_cursor_right(),
            KeyCode::Left => self.try_move_cursor_left(),
            KeyCode::Char(c) => self.try_insert_char(c),
            KeyCode::Backspace => self.backspace(),
            _ => (),
        };
        Ok(None)
    }
}

pub struct IntInputHandler(InputHandler);
#[allow(dead_code)]
impl IntInputHandler {
    pub fn new(initial_number: Option<u32>, max: u32) -> Self {
        Self(InputHandler::new(
            initial_number.map(|n| n.to_string()),
            max.to_string().len(),
            Some(Box::new(move |s: &str| {
                s.parse::<u32>().map_or(false, |n| n <= max)
            })),
        ))
    }

    delegate! {
        to self.0 {
            pub fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>>;
            pub fn value(&self) -> String;
            pub fn cursor_position(&self) -> usize;
            pub fn len(&self) -> usize;
        }
    }
}
