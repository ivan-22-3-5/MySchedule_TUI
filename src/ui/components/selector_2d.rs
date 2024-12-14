pub struct Selector2D {
    row_lengths: Vec<usize>,
    selected_row: usize,
    selected_col: usize,
}
#[allow(dead_code)]
impl Selector2D {
    pub fn new(row_lengths: Vec<usize>) -> Self {
        Self {
            row_lengths,
            selected_row: 0,
            selected_col: 0,
        }
    }
    pub fn move_left(&mut self) {
        let length = self.row_lengths[self.selected_row];
        if length > 1 {
            self.selected_col = match self.selected_col {
                0 => length - 1,
                _ => self.selected_col - 1,
            }
        }
    }

    pub fn move_right(&mut self) {
        let length = self.row_lengths[self.selected_row];
        if length > 1 {
            self.selected_col = (self.selected_col + 1) % length;
        }
    }

    pub fn move_up(&mut self) {
        let length = self.row_lengths.len();
        if length > 1 {
            self.selected_row = match self.selected_row {
                0 => length - 1,
                _ => self.selected_row - 1,
            };
            if self.selected_col >= self.row_lengths[self.selected_row] {
                self.selected_col = self.row_lengths[self.selected_row].saturating_sub(1);
            }
        }
    }

    pub fn move_down(&mut self) {
        let length = self.row_lengths.len();
        if length > 1 {
            self.selected_row = (self.selected_row + 1) % length;
            if self.selected_col >= self.row_lengths[self.selected_row] {
                self.selected_col = self.row_lengths[self.selected_row].saturating_sub(1);
            }
        }
    }

    pub fn selected(&self) -> (usize, usize) {
        (self.selected_row, self.selected_col)
    }
}
