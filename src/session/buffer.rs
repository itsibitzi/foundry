use std::io::{Cursor, Write};

pub struct Buffer {
    text: Cursor<Vec<char>>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            text: Cursor::new(vec![]),
        }
    }

    pub fn insert(&mut self, c: char) {
        self.text.write(c);
    }
}
