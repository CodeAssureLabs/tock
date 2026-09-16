//! Simple counter capsule.

use core::cell::Cell;

pub struct Counter {
    value: Cell<u32>,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: Cell::new(0) }
    }

    pub fn bump(&self) -> u32 {
        self.value.set(self.value.get() + 1);
        self.value.get()
    }
}
