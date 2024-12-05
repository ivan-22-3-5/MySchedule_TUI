use crate::action::Action;
use crate::theme::THEME;
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

pub struct InputField {
    title: String,
    text: String,
    cursor: usize,
    ticks_with_cursor: u8,
    showing_cursor_period: u8,
}

impl InputField {
    #[allow(dead_code)]
    pub fn new(title: String, initial_text: Option<String>) -> Self {
        let initial_text = initial_text.unwrap_or_default();
        let initial_cursor = initial_text.len();
        Self {
            title,
            text: format!("{} ", initial_text),
            cursor: initial_cursor,
            ticks_with_cursor: 0,
            showing_cursor_period: 3,
        }
    }

    fn get_styled_text(&self) -> Line {
        let style_under_cursor = if self.ticks_with_cursor <= self.showing_cursor_period {
            THEME.selected
        } else {
            Style::default()
        };

        Line::from(vec![
            Span::raw(&self.text[..self.cursor]),
            Span::styled(&self.text[self.cursor..self.cursor + 1], style_under_cursor),
            Span::raw(&self.text[self.cursor + 1..]),
        ])
    }

    fn try_move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor = self.cursor.saturating_sub(1);
            self.ticks_with_cursor = 0;
        }
    }

    fn try_move_cursor_right(&mut self) {
        if self.cursor < self.text.len() - 1 {
            self.cursor = self.cursor.saturating_add(1);
            self.ticks_with_cursor = 0;
        }
    }

    fn type_char(&mut self, c: char) {
        self.text.insert(self.cursor, c);
        self.cursor = self.cursor.saturating_add(1);
        self.ticks_with_cursor = 0;
    }

    fn backspace(&mut self) {
        if self.cursor > 0 {
            self.text.remove(self.cursor - 1);
            self.cursor = self.cursor.saturating_sub(1);
        }
    }
}

impl Component for InputField {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Right => self.try_move_cursor_right(),
            KeyCode::Left => self.try_move_cursor_left(),
            KeyCode::Char(c) => self.type_char(c),
            KeyCode::Backspace => self.backspace(),
            _ => (),
        };
        Ok(None)
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        if action == Action::Tick {
            self.ticks_with_cursor += 1;
            if self.ticks_with_cursor == self.showing_cursor_period * 2 {
                self.ticks_with_cursor = 0;
            }
        };
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let text = self.get_styled_text();

        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.clone());
        frame.render_widget(text, block.inner(area));
        frame.render_widget(block, area);
        Ok(())
    }
}
