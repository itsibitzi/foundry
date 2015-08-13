pub struct Buffer {
    // Replace with some other strucutre when perf becomes an issue.
    text: Vec<char>,
    pos: usize,

    line: usize,
    column: usize,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            text: vec![],
            pos: 0,

            line: 0,
            column: 0,
        }
    }

    pub fn prev_column(&mut self) {
        if self.pos > 0 && self.pos < self.text.len() {
            self.pos -= 1;
            self.column -= 1;
            if self.text[self.pos] == '\n' {
                self.line -= 1;
                self.column = 0;
            }
        }
    }

    pub fn next_column(&mut self) {
        if self.pos > 0 && self.pos < self.text.len() {
            self.pos += 1;
            self.column += 1;
            if self.text[self.pos] == '\n' {
                self.line += 1;
                self.column = 0;
            }
        }
    }

    pub fn insert(&mut self, c: char) {
        self.text.insert(self.pos, c);
    }
}
