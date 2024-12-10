use crate::action::Action;
use crate::ui::input::fields::string::InputHandler;
use crate::ui::input::fields::{BorderStyle, InputField};
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use delegate::delegate;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

pub struct IntInputHandler {
    input_handler: InputHandler,
    max: u32,
}

#[allow(dead_code)]
impl IntInputHandler {
    pub fn new(initial_number: Option<u32>, max: u32) -> Self {
        Self {
            input_handler: InputHandler::new(
                initial_number.map(|n| n.to_string()),
                max.to_string().len(),
            ),
            max,
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Char(c) if !c.is_ascii_digit() => (),
            KeyCode::Char(c) => {
                let potential_value: u32 = format!("{}{}", self.input_handler.value(), c)
                    .parse()
                    .expect("IntInputField should always contain a valid number");

                if potential_value <= self.max {
                    self.input_handler.handle_key_event(key)?;
                }
            }
            _ => {
                self.input_handler.handle_key_event(key)?;
            }
        };
        Ok(None)
    }

    delegate! {
        to self.input_handler {
            fn value(&self) -> String;
            fn cursor_position(&self) -> usize;
            fn len(&self) -> usize;
        }
    }
}
pub struct IntInputField {
    title: String,
    is_cursor_visible: bool,
    border_style: Option<BorderStyle>,
    input_handler: IntInputHandler,
}

impl InputField for IntInputField {
    fn get_value(&self) -> String {
        self.input_handler.value()
    }

    fn borders(&mut self, border_style: Option<BorderStyle>) {
        self.border_style = border_style;
    }

    fn set_cursor_visibility(&mut self, visible: bool) {
        self.is_cursor_visible = visible
    }
}
#[allow(dead_code)]
impl IntInputField {
    pub fn new(title: Option<String>, max: u32, initial_number: Option<u32>) -> Self {
        Self {
            title: title.unwrap_or_default(),
            border_style: Some((Borders::ALL, Style::default())),
            is_cursor_visible: false,
            input_handler: IntInputHandler::new(initial_number, max),
        }
    }
}

impl Component for IntInputField {
    delegate! {
        to self.input_handler {
            fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>>;
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let mut area = area;
        if let Some(bs) = self.border_style {
            let block = Block::default()
                .borders(bs.0)
                .border_style(bs.1)
                .title(self.title.clone());
            frame.render_widget(block.clone(), area);
            area = block.inner(area);
        }

        if self.is_cursor_visible {
            frame.set_cursor_position((
                area.x + self.input_handler.cursor_position() as u16,
                area.y,
            ));
        }
        frame.render_widget(Line::from(self.input_handler.value()), area);

        Ok(())
    }
}
