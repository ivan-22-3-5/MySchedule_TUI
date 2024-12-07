use crate::action::Action;
use crate::models::Conference;
use crate::theme::THEME;
use crate::ui::components::{Form, InputField};
use crate::ui::Component;
use crossterm::event::KeyEvent;
use delegate::delegate;
use ratatui::prelude::*;

pub struct ConferenceEditForm(Form);

#[allow(dead_code)]
impl ConferenceEditForm {
    pub fn new(conference: Conference) -> Self {
        let field_layout = vec![
            vec![
                InputField::new("Title".into(), 50, Some(conference.title)),
                InputField::new("Start Time".into(), 5, Some(conference.start_time)),
            ],
            vec![InputField::new("Link".into(), 50, Some(conference.link))],
        ];
        Self(
            Form::new(field_layout)
                .with_field_style(THEME.input_field)
                .with_selected_field_style(THEME.selected_field)
                .with_active_field_style(THEME.active_field),
        )
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
