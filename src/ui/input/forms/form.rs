use crate::action::Action;
use crate::ui::components::Selector2D;
use crate::ui::input::fields::InputField;
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::Borders;

pub struct Form {
    is_selected_field_active: bool,
    selector: Selector2D,
    layout: Vec<Vec<(Box<dyn InputField>, u16)>>,
    field_style: Style,
    selected_field_style: Style,
    active_field_style: Style,
}

#[allow(dead_code)]
impl Form {
    pub fn new<O, I>(layout: O) -> Self
    where
        O: IntoIterator<Item = I>,
        I: IntoIterator<Item = (Box<dyn InputField>, u16)>,
    {
        let layout: Vec<Vec<(Box<dyn InputField>, u16)>> = layout
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();
        Self {
            is_selected_field_active: false,
            selector: Selector2D::new(layout.iter().map(|row| row.len()).collect()),
            layout,
            field_style: Style::default(),
            selected_field_style: Style::default(),
            active_field_style: Style::default(),
        }
    }

    pub fn get_input(&self) -> Vec<Vec<String>> {
        self.layout
            .iter()
            .map(|row| row.iter().map(|(field, _)| field.get_value()).collect())
            .collect()
    }

    //region style setters
    pub fn with_field_style(mut self, style: Style) -> Self {
        self.field_style = style;
        self
    }

    pub fn with_selected_field_style(mut self, style: Style) -> Self {
        self.selected_field_style = style;
        self
    }

    pub fn with_active_field_style(mut self, style: Style) -> Self {
        self.active_field_style = style;
        self
    }
    //endregion style setters

    fn handle_field_selection(&mut self, key: KeyEvent) {
        let field_style = self.field_style;
        self.selected_field().borders((Borders::ALL, field_style));
        match key.code {
            KeyCode::Up => self.selector.move_up(),
            KeyCode::Down => self.selector.move_down(),
            KeyCode::Left => self.selector.move_left(),
            KeyCode::Right => self.selector.move_right(),
            _ => {}
        }
        let selected_field_style = self.selected_field_style;
        self.selected_field()
            .borders((Borders::ALL, selected_field_style))
    }

    fn propagate_key(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        if self.is_selected_field_active {
            self.selected_field().handle_key_event(key)?;
        }
        Ok(None)
    }

    fn selected_field(&mut self) -> &mut Box<dyn InputField> {
        let (row, col) = self.selector.selected();
        &mut self.layout[row][col].0
    }

    fn build_layout(&self, area: Rect) -> Vec<Vec<Rect>> {
        let length = self.layout.len();
        let outer_layout = Layout::vertical((0..length).map(|_| Constraint::Length(3))).split(area);

        self.layout
            .iter()
            .enumerate()
            .map(|(index, row)| {
                Layout::horizontal(row.iter().map(|(_, width)| Constraint::Length(*width)))
                    .split(outer_layout[index])
                    .to_vec()
            })
            .collect()
    }
}

impl Component for Form {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        if self.is_selected_field_active {
            match key.code {
                KeyCode::Esc => {
                    self.is_selected_field_active = false;
                    let selected_field_style = self.selected_field_style;
                    self.selected_field()
                        .borders((Borders::ALL, selected_field_style));
                    self.selected_field().set_cursor_visibility(false);
                }
                _ => {
                    self.propagate_key(key)?;
                }
            }
        } else {
            match key.code {
                KeyCode::Enter => {
                    self.is_selected_field_active = true;
                    let active_field_style = self.active_field_style;
                    self.selected_field()
                        .borders((Borders::ALL, active_field_style));
                    self.selected_field().set_cursor_visibility(true);
                }
                _ => {
                    self.handle_field_selection(key);
                }
            }
        }
        Ok(None)
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        if self.is_selected_field_active {
            self.selected_field().update(action)?;
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let layout = self.build_layout(area);

        for (row_index, row) in self.layout.iter_mut().enumerate() {
            for (col_index, (field, _)) in row.iter_mut().enumerate() {
                field.draw(frame, layout[row_index][col_index])?;
            }
        }
        Ok(())
    }
}
