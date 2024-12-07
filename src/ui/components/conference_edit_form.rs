use crossterm::event::KeyEvent;
use ratatui::prelude::*;

use crate::action::Action;
use crate::models::Conference;
use crate::theme::THEME;
use crate::ui::components::{Form, InputField};
use crate::ui::Component;

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
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        self.0.handle_key_event(key)?;
        Ok(None)
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        self.0.update(action)?;
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        self.0.draw(frame, area)?;
        Ok(())
    }
}
