use crate::action::Action;
use crate::ui::input::fields::{BorderStyle, InputField, InputHandler};
use crate::ui::Component;
use crossterm::event::KeyEvent;
use delegate::delegate;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

pub struct StrInputField {
    title: String,
    is_cursor_visible: bool,
    border_style: Option<BorderStyle>,
    input_handler: InputHandler,
    left_padding: u16,
}

impl InputField for StrInputField {
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
impl StrInputField {
    pub fn new(title: Option<String>, max_length: usize, initial_text: Option<String>) -> Self {
        Self {
            title: title.unwrap_or_default(),
            border_style: Some((Borders::ALL, Style::default())),
            is_cursor_visible: false,
            input_handler: InputHandler::new(initial_text, max_length, None),
            left_padding: 0,
        }
    }

    fn recalculate_padding(&mut self, width: u16) {
        let text_len = self.input_handler.len() as u16;
        let cursor_pos = self.input_handler.cursor_position() as u16;

        if !(self.left_padding..(self.left_padding + width)).contains(&cursor_pos) {
            self.left_padding = self
                .left_padding
                .max(cursor_pos.saturating_sub(width)) // if cursor is out of right bound set padding to cursor_pos - width
                .min(cursor_pos); // if cursor is out of left bound set padding to cursor_pos
        }
        self.left_padding = self.left_padding.min(text_len.saturating_sub(width));
        // move padding to the left if there is empty space
    }
}

impl Component for StrInputField {
    delegate! {
        to self.input_handler {
            fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>>;
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let mut area = Layout::vertical([Constraint::Length(3)]).split(area)[0];
        if let Some(bs) = self.border_style {
            let block = Block::default()
                .borders(bs.0)
                .border_style(bs.1)
                .title(self.title.clone());
            frame.render_widget(block.clone(), area);
            area = block.inner(area);
        }

        self.recalculate_padding(area.width - 1);

        if self.is_cursor_visible {
            frame.set_cursor_position((
                area.x + self.input_handler.cursor_position() as u16 - self.left_padding,
                area.y,
            ));
        }
        let string_slice_to_show: String = self
            .input_handler
            .value()
            .chars()
            .skip(self.left_padding as usize)
            .take(area.width as usize)
            .collect();
        frame.render_widget(Line::from(string_slice_to_show), area);
        Ok(())
    }
}
