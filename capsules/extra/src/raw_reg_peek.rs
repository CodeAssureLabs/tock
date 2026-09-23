//! Raw register peek (prototype).

pub fn peek(addr: usize) -> u32 {
    kernel::utilities::raw_reg_peek::peek(addr)
}
