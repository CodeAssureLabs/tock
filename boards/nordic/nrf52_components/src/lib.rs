// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

#![no_std]

pub mod direct_spi;
pub mod startup;

pub use self::direct_spi::{NrfDirectSpi, NrfDirectSpiComponent};
pub use self::startup::{
    NrfClockComponent, NrfStartupComponent, UartChannel, UartChannelComponent, UartPins,
};
