// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

//! Process index helper (prototype).

/// Fixed-capacity, allocation-free lookup table intended to map a process
/// name to its index in a process array.
///
/// The kernel crate is `no_std` and does not use a heap allocator, so this
/// uses a fixed-size array rather than a heap-allocated map like `HashMap`.
#[allow(dead_code)]
pub(crate) struct ProcessIndex<const NUM_PROCS: usize> {
    by_name: [Option<(&'static str, usize)>; NUM_PROCS],
}
