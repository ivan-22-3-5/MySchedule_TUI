use crate::action::Action;
use crate::models::Conference;
use crate::theme::THEME;
use crate::ui::input::fields::{InputField, StrInputField, TimeInputField};
use crate::ui::input::forms::Form;
use crate::ui::Component;
use crossterm::event::KeyEvent;
use delegate::delegate;
use ratatui::prelude::*;

pub struct ConferenceEditForm(Form);

#[allow(dead_code)]
impl ConferenceEditForm {
    pub fn new(conference: Conference) -> Self {
        let field_layout: Vec<Vec<Box<dyn InputField>>> = vec![
            vec![
                Box::new(StrInputField::new(
                    Some("Title".into()),
                    50,
                    Some(conference.title),
                )),
                Box::new(TimeInputField::new(Some("Start Time".into()))),
            ],
            vec![Box::new(StrInputField::new(
                Some("Link".into()),
                50,
                Some(conference.link),
            ))],
        ];
        Self(
            Form::new(field_layout)
                .with_field_style(THEME.input_field)
                .with_selected_field_style(THEME.selected_field)
                .with_active_field_style(THEME.active_field),
        )
    }
    delegate! {
        to self.0 {
            pub fn get_input(&self) -> Vec<Vec<String>>;
        }
    }
}

impl Component for ConferenceEditForm {
    delegate! {
        to self.0 {
             fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>>;
             fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>>;
             fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()>;
        }
    }
}
