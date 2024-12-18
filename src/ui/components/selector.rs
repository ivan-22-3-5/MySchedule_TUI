pub struct Selector {
    length: usize,
    pub index: usize,
}

impl Selector {
    pub fn new(length: usize) -> Self {
        Self { length, index: 0 }
    }
    pub fn prev(&mut self) {
        if self.length > 1 {
            self.index = match self.index {
                0 => self.length - 1,
                _ => self.index - 1,
            };
        }
    }

    pub fn next(&mut self) {
        if self.length > 1 {
            self.index = (self.index + 1) % self.length;
        }
    }
}
