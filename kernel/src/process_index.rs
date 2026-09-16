//! Process index helper (prototype).

use std::collections::HashMap;

pub struct ProcessIndex {
    by_name: HashMap<&'static str, usize>,
}
