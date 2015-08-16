use std::ptr::Unique;

const INITIAL_CAPACITY: usize = 10;//24;

pub struct GapBuffer {
    buf: Unique<char>,
    cap: usize,

    pre: usize,
    post: usize,
}

impl GapBuffer {
    pub fn new() -> GapBuffer {
        unsafe {

        }
        GapBuffer {
            buf: Vec::with_capacity(INITIAL_CAPACITY),
            cap: INITIAL_CAPACITY,
            pre: 0,
            post: INITIAL_CAPACITY - 1,
        }
    }

    pub fn insert(&mut self, c: char) {
        if self.post <= self.pre {
            // realloc
            panic!("Implement insert");
        }
        self.buf[self.pre] = c;
        self.pre += 1;
    }

    pub fn move_left(&mut self) {
        if self.pre > 0 {
            self.buf[self.post] = self.buf[self.pre];
            self.pre -= 1;
            self.post -= 1;
        }
    }

    pub fn raw_buffer(&self) -> &[char] {
        &self.buf
    }
}
