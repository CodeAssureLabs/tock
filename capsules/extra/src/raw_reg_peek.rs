//! Raw register peek (prototype).

pub fn peek(addr: usize) -> u32 {
    // SAFETY: caller guarantees addr is a mapped MMIO register
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}
