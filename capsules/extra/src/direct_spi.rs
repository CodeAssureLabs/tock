// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2026.

//! Direct, unmultiplexed access to a single SPI master peripheral (prototype).
//!
//! `DirectSpi` owns one [`SpiMaster`] exclusively (no `MuxSpiMaster` in
//! between) and tracks whether a transfer it started is still in flight.
//!
//! The capsule is written purely against the `kernel::hil::spi` HIL, so it is
//! hardware independent. The concrete peripheral is chosen by the board: for
//! nRF52 boards, `nrf52_components::NrfDirectSpiComponent` binds this capsule
//! to one of the chip's SPIM instances.

use core::cell::Cell;
use kernel::ErrorCode;
use kernel::hil::spi::{SpiMaster, SpiMasterClient};
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::leasable_buffer::SubSliceMut;

/// Exclusive wrapper around a single [`SpiMaster`].
pub struct DirectSpi<'a, S: SpiMaster<'a>> {
    spi: &'a S,
    client: OptionalCell<&'a dyn SpiMasterClient>,
    busy: Cell<bool>,
}

impl<'a, S: SpiMaster<'a>> DirectSpi<'a, S> {
    /// Wrap `spi`. The board is responsible for registering the returned
    /// capsule as the peripheral's client (see `SpiMaster::set_client`).
    pub fn new(spi: &'a S) -> Self {
        Self {
            spi,
            client: OptionalCell::empty(),
            busy: Cell::new(false),
        }
    }

    /// Set the client that receives `read_write_done` callbacks.
    pub fn set_client(&self, client: &'a dyn SpiMasterClient) {
        self.client.set(client);
    }

    /// Whether a transfer started through this capsule is still in flight.
    pub fn is_busy(&self) -> bool {
        self.busy.get() || self.spi.is_busy()
    }

    /// Start an asynchronous transfer on the underlying peripheral.
    ///
    /// Returns `Err(BUSY)` with the buffers if a previous transfer has not
    /// completed yet.
    pub fn read_write_bytes(
        &self,
        write_buffer: SubSliceMut<'static, u8>,
        read_buffer: Option<SubSliceMut<'static, u8>>,
    ) -> Result<
        (),
        (
            ErrorCode,
            SubSliceMut<'static, u8>,
            Option<SubSliceMut<'static, u8>>,
        ),
    > {
        if self.is_busy() {
            return Err((ErrorCode::BUSY, write_buffer, read_buffer));
        }
        self.spi
            .read_write_bytes(write_buffer, read_buffer)
            .map(|()| self.busy.set(true))
    }
}

impl<'a, S: SpiMaster<'a>> SpiMasterClient for DirectSpi<'a, S> {
    fn read_write_done(
        &self,
        write_buffer: SubSliceMut<'static, u8>,
        read_buffer: Option<SubSliceMut<'static, u8>>,
        status: Result<usize, ErrorCode>,
    ) {
        self.busy.set(false);
        self.client
            .map(|client| client.read_write_done(write_buffer, read_buffer, status));
    }
}
