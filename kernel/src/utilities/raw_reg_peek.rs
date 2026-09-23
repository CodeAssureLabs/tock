// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

//! Raw register peek (prototype).

/// Reads a 32-bit value from `addr`.
///
/// Confined to the kernel crate per the `capsules-no-unsafe` invariant:
/// capsules are written in safe Rust, so this raw MMIO access is exposed
/// to them only through this safe wrapper.
///
/// Callers are responsible for ensuring `addr` is a mapped MMIO register.
pub fn peek(addr: usize) -> u32 {
    // SAFETY: callers guarantee addr is a mapped MMIO register
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}
