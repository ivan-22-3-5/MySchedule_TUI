use crate::action::Action;
use crate::models::Conference;
use crate::theme::THEME;
use crate::ui::input::fields::{CarouselInputField, InputField, StrInputField, TimeInputField};
use crate::ui::input::forms::Form;
use crate::ui::Component;
use crossterm::event::KeyEvent;
use delegate::delegate;
use ratatui::prelude::*;

pub struct ConferenceEditForm(Form);

impl ConferenceEditForm {
    const AUTOSTART_PERMISSION_OPTIONS: [&'static str; 2] = ["Deny", "Allow"];

    pub fn new(conference: Conference) -> Self {
        let field_layout: Vec<Vec<(Box<dyn InputField>, u16)>> = vec![
            vec![(
                Box::new(StrInputField::new(
                    Some("Title".into()),
                    50,
                    Some(conference.title),
                )),
                50,
            )],
            vec![
                (
                    Box::new(TimeInputField::new(
                        Some("Start Time".into()),
                        Some(conference.start_time),
                    )),
                    25,
                ),
                (
                    Box::new(TimeInputField::new(
                        Some("End Time".into()),
                        Some(conference.end_time),
                    )),
                    25,
                ),
            ],
            vec![(
                Box::new(StrInputField::new(
                    Some("Link".into()),
                    50,
                    Some(conference.link),
                )),
                50,
            )],
            vec![(
                Box::new(StrInputField::new(
                    Some("Password".into()),
                    50,
                    Some(conference.password.unwrap_or_default()),
                )),
                50,
            )],
            vec![(
                Box::new(CarouselInputField::new(
                    Some("Autostart".into()),
                    Self::AUTOSTART_PERMISSION_OPTIONS
                        .map(|o| o.to_string())
                        .to_vec(),
                    conference.autostart_permission as usize,
                )),
                50,
            )],
        ];
        Self(
            Form::new(field_layout)
                .with_field_style(THEME.input_field)
                .with_selected_field_style(THEME.selected_field)
                .with_active_field_style(THEME.active_field),
        )
    }

    pub fn get_conference(&self) -> Conference {
        let input = self.0.get_input();
        Conference {
            title: input[0][0].clone(),
            start_time: input[1][0]
                .parse()
                .expect("TimeInputField should give valid time"),
            end_time: input[1][1]
                .parse()
                .expect("TimeInputField should give valid time"),
            link: input[2][0].clone(),
            password: (!input[3][0].is_empty()).then_some(input[3][0].clone()),
            autostart_permission: input[4][0] == Self::AUTOSTART_PERMISSION_OPTIONS[1],
            ..Default::default()
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
