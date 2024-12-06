use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;

use crate::action::Action;
use crate::models::Conference;
use crate::theme::THEME;
use crate::ui::components::{InputField, Selector2D};
use crate::ui::Component;

pub struct ConferenceEditForm {
    title: InputField,
    link: InputField,
    start_time: InputField,
    active_field: Option<Field>,
    selector: Selector2D,
    layout: Vec<Vec<Field>>,
}

#[derive(Copy, Clone)]
enum Field {
    Title,
    Link,
    StartTime,
}
#[allow(dead_code)]
impl ConferenceEditForm {
    pub fn new(conference: Conference) -> Self {
        let field_layout = vec![vec![Field::Title, Field::StartTime], vec![Field::Link]];
        Self {
            title: InputField::new("Title".into(), 50, Some(conference.title)),
            link: InputField::new("Link".into(), 50, Some(conference.link)),
            start_time: InputField::new("Start time".into(), 5, Some(conference.start_time)),
            active_field: None,
            selector: Selector2D::new(field_layout.iter().map(|row| row.len()).collect()),
            layout: field_layout,
        }
    }

    fn handle_transition(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.selector.move_up(),
            KeyCode::Down => self.selector.move_down(),
            KeyCode::Left => self.selector.move_left(),
            KeyCode::Right => self.selector.move_right(),

            KeyCode::Enter => {
                self.active_field = Some(self.selected_field());
            }
            _ => {}
        }
    }

    fn selected_field(&self) -> Field {
        let (row, col) = self.selector.selected();
        self.layout[row][col]
    }

    fn build_layout(&self, area: Rect) -> Vec<Vec<Rect>> {
        let length = self.layout.len();
        let outer_layout =
            Layout::vertical((0..length).map(|_| Constraint::Ratio(1, length as u32))).split(area);

        self.layout
            .iter()
            .enumerate()
            .map(|(index, row)| {
                Layout::horizontal((0..row.len()).map(|_| Constraint::Ratio(1, row.len() as u32)))
                    .split(outer_layout[index])
                    .to_vec()
            })
            .collect()
    }
}

impl Component for ConferenceEditForm {
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match &self.active_field {
            None => self.handle_transition(key),
            Some(field) => match key.code {
                KeyCode::Esc => {
                    self.active_field = None;
                }
                _ => {
                    match field {
                        Field::Title => self.title.handle_key_event(key)?,
                        Field::Link => self.link.handle_key_event(key)?,
                        Field::StartTime => self.start_time.handle_key_event(key)?,
                    };
                }
            },
        };
        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match &self.active_field {
            None => {}
            Some(field) => {
                match field {
                    Field::Title => self.title.update(action)?,
                    Field::Link => self.link.update(action)?,
                    Field::StartTime => self.start_time.update(action)?,
                };
            }
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = self.build_layout(area);
        self.title.border_style(THEME.input_field);
        self.link.border_style(THEME.input_field);
        self.start_time.border_style(THEME.input_field);
        match &self.active_field {
            None => match self.selected_field() {
                Field::Title => self.title.border_style(THEME.selected_field),
                Field::Link => self.link.border_style(THEME.selected_field),
                Field::StartTime => self.start_time.border_style(THEME.selected_field),
            },
            Some(field) => match field {
                Field::Title => self.title.border_style(THEME.active_field),
                Field::Link => self.link.border_style(THEME.active_field),
                Field::StartTime => self.start_time.border_style(THEME.active_field),
            },
        }
        self.title.draw(frame, layout[0][0])?;
        self.start_time.draw(frame, layout[0][1])?;
        self.link.draw(frame, layout[1][0])?;
        Ok(())
    }
}
