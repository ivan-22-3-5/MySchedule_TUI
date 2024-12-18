use crate::action::Action;
use crate::ui::components::Selector;
use crate::ui::input::fields::{BorderStyle, InputField};
use crate::ui::Component;
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, Style};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

pub struct CarouselInputField {
    title: String,
    options: Vec<String>,
    selector: Selector,
    border_style: BorderStyle,
}

impl CarouselInputField {
    pub fn new(title: Option<String>, options: Vec<String>) -> Self {
        Self {
            selector: Selector::new(options.len()),
            title: title.unwrap_or_default(),
            options,
            border_style: (Borders::ALL, Style::default()),
        }
    }
}

impl Component for CarouselInputField {
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Up | KeyCode::Right => self.selector.next(),
            KeyCode::Down | KeyCode::Left => self.selector.prev(),
            _ => (),
        };
        Ok(None)
    }
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        fn calc_padding(width: usize, text_len: usize) -> (usize, usize) {
            let width = width - 2 - 2;
            let total_padding = width - text_len;
            (total_padding / 2, total_padding - total_padding / 2)
        }

        let area = Layout::vertical([Constraint::Length(3)]).split(area)[0];
        let block = Block::default()
            .borders(self.border_style.0)
            .border_style(self.border_style.1)
            .title(self.title.clone());
        frame.render_widget(block.clone(), area);

        let (left_padding, right_padding) =
            calc_padding(area.width as usize, self.options[self.selector.index].len());
        frame.render_widget(
            Line::from(format!(
                "<{}{}{}>",
                " ".repeat(left_padding),
                self.options[self.selector.index].clone(),
                " ".repeat(right_padding),
            )),
            block.inner(area),
        );
        Ok(())
    }
}

impl InputField for CarouselInputField {
    fn get_value(&self) -> String {
        self.options[self.selector.index].clone()
    }

    fn borders(&mut self, border_style: BorderStyle) {
        self.border_style = border_style;
    }

    fn set_cursor_visibility(&mut self, visible: bool) {
        let _ = visible;
    }
}
