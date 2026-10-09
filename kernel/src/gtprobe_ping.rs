// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! Probe module: bounces a counter to [`crate::gtprobe_pong`].

use crate::gtprobe_pong::pong;

/// Decrements `remaining` and hands the turn over to [`pong`] until it reaches zero.
pub fn ping(remaining: u32) -> u32 {
    if remaining == 0 {
        return 0;
    }
    pong(remaining - 1) + 1
}
