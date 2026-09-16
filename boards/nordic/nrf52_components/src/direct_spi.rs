// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

//! Component binding the hardware-independent `DirectSpi` capsule to an
//! nRF52 `SPIM` peripheral.
//!
//! This is the only place that knows the capsule is running on nRF52: it
//! routes the SPIM signals to the requested pins, registers the capsule as
//! the peripheral's client and initializes the peripheral.
//!
//! Usage
//! -----
//! ```rust,ignore
//! let direct_spi = nrf52_components::NrfDirectSpiComponent::new(
//!     &base_peripherals.spim0,
//!     SPI_MOSI,
//!     SPI_MISO,
//!     SPI_CLK,
//! )
//! .finalize(nrf52_components::nrf_direct_spi_component_static!());
//! ```

use capsules_extra::direct_spi::DirectSpi;
use core::mem::MaybeUninit;
use kernel::component::Component;
use kernel::hil::spi::SpiMaster;
use nrf52::gpio::Pin;
use nrf52::pinmux::Pinmux;
use nrf52::spi::SPIM;

/// The `DirectSpi` capsule specialized to an nRF52 SPIM instance.
pub type NrfDirectSpi = DirectSpi<'static, SPIM<'static>>;

#[macro_export]
macro_rules! nrf_direct_spi_component_static {
    () => {{ kernel::static_buf!($crate::direct_spi::NrfDirectSpi) }};
}

pub struct NrfDirectSpiComponent {
    spim: &'static SPIM<'static>,
    mosi: Pin,
    miso: Pin,
    sck: Pin,
}

impl NrfDirectSpiComponent {
    pub fn new(spim: &'static SPIM<'static>, mosi: Pin, miso: Pin, sck: Pin) -> Self {
        Self {
            spim,
            mosi,
            miso,
            sck,
        }
    }
}

impl Component for NrfDirectSpiComponent {
    type StaticInput = &'static mut MaybeUninit<NrfDirectSpi>;
    type Output = &'static NrfDirectSpi;

    fn finalize(self, static_buffer: Self::StaticInput) -> Self::Output {
        self.spim.configure(
            Pinmux::new(self.mosi),
            Pinmux::new(self.miso),
            Pinmux::new(self.sck),
        );

        let direct_spi = static_buffer.write(DirectSpi::new(self.spim));
        self.spim.set_client(direct_spi);
        let _ = self.spim.init();

        direct_spi
    }
}
