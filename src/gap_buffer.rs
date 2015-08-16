use std::ptr::Unique;
use std::mem;

use alloc::heap;
use alloc::oom;

const INITIAL_CAPACITY: usize = 10;//24;

pub struct GapBuffer {
    buf: Unique<char>,
    cap: usize,

    pre: usize,
    post: usize,
}

impl GapBuffer {
    pub fn new() -> GapBuffer {

        GapBuffer {
            buf: unsafe {
                let alloc_size = INITIAL_CAPACITY.checked_mul(mem::size_of::<char>()).expect("gapbuffer capacity overflow");
                let align = mem::align_of::<char>();
                let ptr = heap::allocate(alloc_size, align);

                if ptr.is_null() {
                    oom();
                }
                Unique::new(ptr as *mut char)
            },
            cap: INITIAL_CAPACITY,
            pre: 0,
            post: INITIAL_CAPACITY - 1,
        }
    }

    pub fn insert(&mut self, c: char) {
        unimplemented!();
    }

    pub fn move_left(&mut self) {
        unimplemented!();
    }
}

impl Drop for GapBuffer {
    fn drop(&mut self) {
        unsafe {
            heap::deallocate(*self.buf as *mut _, self.cap * mem::size_of::<char>(), mem::align_of::<char>());
        }
    }
}
