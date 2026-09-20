// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

//! Raw register peek (prototype).
//!
//! Capsules are hardware-independent and written in safe Rust, so this
//! capsule never dereferences a raw address itself. Instead the caller (a
//! chip or board crate) constructs a [`StaticRef`] to a mapped MMIO register
//! and passes it in; the read itself goes through the register interface.

use kernel::utilities::registers::interfaces::Readable;
use kernel::utilities::registers::ReadOnly;
use kernel::utilities::StaticRef;

/// Read the current value of a memory-mapped 32-bit register.
pub fn peek(reg: StaticRef<ReadOnly<u32>>) -> u32 {
    reg.get()
}
