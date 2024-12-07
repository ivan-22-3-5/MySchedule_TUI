use crate::action::Action;
use crate::ui::input::fields::{InputField, StrInputField};
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use delegate::delegate;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::Frame;

pub struct IntInputField {
    max: u32,
    field: StrInputField,
}

impl InputField for IntInputField {
    delegate! {
        to self.field {
            fn get_value(&self) -> String;
            fn border_style(&mut self, style: Style);
        }
    }
}
#[allow(dead_code)]
impl IntInputField {
    pub fn new(title: String, max: u32, initial_number: Option<u32>) -> Self {
        let initial_text = match initial_number {
            Some(n) => n.to_string(),
            None => "".into(),
        };

        Self {
            field: StrInputField::new(title, max.to_string().len(), Some(initial_text)),
            max,
        }
    }
}

impl Component for IntInputField {
    delegate! {
        to self.field {
            fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>>;
            fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()>;
        }
    }
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Char(c) if !c.is_ascii_digit() => Ok(None),
            KeyCode::Char(c) => {
                let potential_value: u32 = format!("{}{}", self.field.get_value(), c)
                    .parse()
                    .expect("IntInputField should always contain a valid number");

                if potential_value <= self.max {
                    self.field.handle_key_event(key)?;
                }
                Ok(None)
            }
            _ => Ok(self.field.handle_key_event(key)?),
        }
    }
}
