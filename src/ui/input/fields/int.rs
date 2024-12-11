use crate::action::Action;
use crate::ui::input::fields::{BorderStyle, InputField, IntInputHandler};
use crate::ui::Component;
use crossterm::event::KeyEvent;
use delegate::delegate;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

pub struct IntInputField {
    title: String,
    is_cursor_visible: bool,
    border_style: Option<BorderStyle>,
    input_handler: IntInputHandler,
}

impl InputField for IntInputField {
    fn get_value(&self) -> String {
        self.input_handler.value().to_string()
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
        let mut area = Layout::vertical([Constraint::Length(3)]).split(area)[0];
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
        frame.render_widget(Line::from(self.input_handler.value().to_string()), area);

        Ok(())
    }
}
