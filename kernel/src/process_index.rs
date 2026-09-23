//! Process index helper (prototype).
//!
//! The kernel is `no_std`, so the index is a fixed-capacity array sized by a
//! const generic, mirroring `ProcessArray`.

/// Maps process names to their slot in the process array.
pub struct ProcessIndex<const NUM_PROCS: usize> {
    by_name: [Option<(&'static str, usize)>; NUM_PROCS],
}

impl<const NUM_PROCS: usize> ProcessIndex<NUM_PROCS> {
    /// Create an empty index.
    pub const fn new() -> Self {
        Self {
            by_name: [None; NUM_PROCS],
        }
    }

    /// Record `name` at `slot`. Returns `false` if the index is full.
    pub fn insert(&mut self, name: &'static str, slot: usize) -> bool {
        match self.by_name.iter_mut().find(|e| e.is_none()) {
            Some(entry) => {
                *entry = Some((name, slot));
                true
            }
            None => false,
        }
    }

    /// Look up the slot recorded for `name`.
    pub fn get(&self, name: &str) -> Option<usize> {
        self.by_name
            .iter()
            .flatten()
            .find(|(n, _)| *n == name)
            .map(|(_, slot)| *slot)
    }
}
