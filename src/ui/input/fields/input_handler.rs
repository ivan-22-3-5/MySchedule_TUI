pub struct InputHandler {
    text: Vec<char>,
    cursor: usize,
    max_length: usize,
}

#[allow(dead_code)]
impl InputHandler {
    pub fn new(initial_text: Option<String>, max_length: usize) -> Self {
        let initial_text: Vec<char> = initial_text
            .unwrap_or_default()
            .chars()
            .take(max_length)
            .collect();
        Self {
            max_length,
            cursor: initial_text.len(),
            text: initial_text,
        }
    }

    pub fn value(&self) -> String {
        self.text.iter().collect()
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn try_move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor = self.cursor.saturating_sub(1);
        }
    }

    pub fn try_move_cursor_right(&mut self) {
        if self.cursor < self.text.len() {
            self.cursor = self.cursor.saturating_add(1);
        }
    }

    pub fn type_char(&mut self, c: char) {
        if self.text.len() < self.max_length {
            self.text.insert(self.cursor, c);
            self.cursor = self.cursor.saturating_add(1);
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.text.remove(self.cursor - 1);
            self.cursor = self.cursor.saturating_sub(1);
        }
    }
}
