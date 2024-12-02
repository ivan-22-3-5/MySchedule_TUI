pub struct Selector {
    length: u64,
    pub index: u64,
}

impl Selector {
    pub fn new(length: u64) -> Self {
        Self { length, index: 0 }
    }
    pub fn prev(&mut self) {
        if self.length > 1 {
            self.index = (self.index + (self.length - 1)) % self.length;
        }
    }

    pub fn next(&mut self) {
        if self.length > 1 {
            self.index = (self.index + 1) % self.length;
        }
    }
}
