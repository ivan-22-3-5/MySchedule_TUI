use crate::action::Action;
use crate::ui::input::fields::{InputField, StrInputField};
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use delegate::delegate;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::Frame;

pub struct IntInputField(StrInputField);

impl InputField for IntInputField {
    delegate! {
        to self.0 {
            fn get_value(&self) -> String;
            fn border_style(&mut self, style: Style);
        }
    }
}
#[allow(dead_code)]
impl IntInputField {
    pub fn new(title: String, max_length: usize, initial_number: Option<u32>) -> Self {
        let initial_text = match initial_number {
            Some(n) => n.to_string(),
            None => "".into(),
        };

        Self(StrInputField::new(title, max_length, Some(initial_text)))
    }
}

impl Component for IntInputField {
    delegate! {
        to self.0 {
            fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>>;
            fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()>;
        }
    }
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Char(c) if !c.is_ascii_digit() => Ok(None),
            _ => Ok(self.0.handle_key_event(key)?),
        }
    }
}
