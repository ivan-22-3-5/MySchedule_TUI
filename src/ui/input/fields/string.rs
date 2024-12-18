use crate::action::Action;
use crate::ui::input::fields::{BaseInputField, BaseInputHandler, BorderStyle, InputField};
use crate::ui::Component;
use crossterm::event::KeyEvent;
use delegate::delegate;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct StrInputField(BaseInputField);

impl InputField for StrInputField {
    delegate! {
        to self.0 {
            fn get_value(&self) -> String;
            fn borders(&mut self, border_style: BorderStyle);
            fn set_cursor_visibility(&mut self, visible: bool);
        }
    }
}

impl StrInputField {
    pub fn new(title: Option<String>, max_length: usize, initial_text: Option<String>) -> Self {
        Self(BaseInputField::new(
            title,
            Box::new(BaseInputHandler::new(initial_text, max_length, None)),
        ))
    }
}

impl Component for StrInputField {
    delegate! {
        to self.0 {
            fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>>;
            fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()>;
        }
    }
}
