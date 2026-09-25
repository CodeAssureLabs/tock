//! Raw register peek (prototype).

/// Reads a volatile 32-bit value from an MMIO register at the given address.
///
/// # Safety
///
/// The caller must guarantee that `addr` points to a valid, mapped MMIO register.
/// The address must be properly aligned (4-byte boundary) and remain valid for the
/// duration of the read operation.
pub fn peek(addr: usize) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}
