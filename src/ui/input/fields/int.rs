use crate::action::Action;
use crate::ui::input::fields::{
    BaseInputField, BaseInputHandler, BorderStyle, InputField, InputHandler,
};
use crate::ui::Component;
use crossterm::event::KeyEvent;
use delegate::delegate;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct IntInputHandler(BaseInputHandler);

impl IntInputHandler {
    pub fn new(initial_number: Option<u32>, max: u32) -> Self {
        Self(BaseInputHandler::new(
            initial_number.map(|n| n.to_string()),
            max.to_string().len(),
            Some(Box::new(move |s: &str| {
                s.parse::<u32>().is_ok_and(|n| n <= max)
            })),
        ))
    }
}

impl InputHandler for IntInputHandler {
    fn value(&self) -> String {
        let value = self.0.value();
        if value.is_empty() {
            String::from("0")
        } else {
            value
        }
    }
    delegate! {
        to self.0 {
            fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>>;
            fn cursor_position(&self) -> usize;
            fn len(&self) -> usize;
        }
    }
}

pub struct IntInputField(BaseInputField);

impl InputField for IntInputField {
    delegate! {
        to self.0 {
            fn get_value(&self) -> String;
            fn borders(&mut self, border_style: BorderStyle);
            fn set_cursor_visibility(&mut self, visible: bool);
        }
    }
}
#[allow(dead_code)]
impl IntInputField {
    pub fn new(title: Option<String>, max: u32, initial_number: Option<u32>) -> Self {
        Self(BaseInputField::new(
            title,
            Box::new(IntInputHandler::new(initial_number, max)),
        ))
    }
}

impl Component for IntInputField {
    delegate! {
        to self.0 {
            fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>>;
            fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()>;
        }
    }
}
