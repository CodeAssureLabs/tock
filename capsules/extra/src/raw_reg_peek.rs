// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

//! Raw register peek (prototype).
//!
//! Reads a single 32-bit memory-mapped register. Capsules are untrusted and
//! forbid `unsafe`, so this capsule cannot dereference a raw address itself.
//! Instead, the trusted chip or board crate constructs a [`StaticRef`] to the
//! register (documenting why that address is valid MMIO) and hands it to this
//! capsule, which then performs the read through the safe register interface.

use kernel::utilities::StaticRef;
use kernel::utilities::registers::ReadOnly;
use kernel::utilities::registers::interfaces::Readable;

/// Read the current value of the given memory-mapped register.
pub fn peek(reg: StaticRef<ReadOnly<u32>>) -> u32 {
    reg.get()
}
