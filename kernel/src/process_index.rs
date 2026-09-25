// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

//! Helper for looking up processes in a process table by name.
//!
//! The kernel has no heap allocator, so this helper does not build a separate
//! map of names. It wraps a slice of [`ProcessSlot`]s and performs a linear
//! scan, which matches how the rest of the kernel traverses the process table.

use crate::process::ProcessSlot;

/// View of a process table that supports lookup by process name.
pub struct ProcessIndex<'a> {
    processes: &'a [ProcessSlot],
}

impl<'a> ProcessIndex<'a> {
    /// Create an index over the given process table.
    pub const fn new(processes: &'a [ProcessSlot]) -> Self {
        Self { processes }
    }

    /// Return the position in the process table of the first loaded process
    /// with the given name, or `None` if no loaded process has that name.
    pub fn index_by_name(&self, name: &str) -> Option<usize> {
        self.processes
            .iter()
            .position(|slot| slot.get().is_some_and(|p| p.get_process_name() == name))
    }
}
