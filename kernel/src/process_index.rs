//! Process index helper (prototype).

use alloc::collections::HashMap;

pub struct ProcessIndex {
    by_name: HashMap<&'static str, usize>,
}
