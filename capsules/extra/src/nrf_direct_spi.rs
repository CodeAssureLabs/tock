//! Direct SPIM access for nRF52 (prototype).

use kernel::utilities::cells::OptionalCell;
use nrf52::spi::SPIM;

pub struct NrfDirectSpi<'a> {
    spim: &'a SPIM<'a>,
    busy: OptionalCell<bool>,
}

impl<'a> NrfDirectSpi<'a> {
    pub fn new(spim: &'a SPIM<'a>) -> Self {
        Self { spim, busy: OptionalCell::empty() }
    }
}
