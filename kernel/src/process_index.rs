//! Process index helper (prototype).

/// Fixed-size table mapping process names to their index in the process
/// array. Uses a plain array rather than a heap-backed map because the
/// kernel is `no_std` and does not use `alloc`.
pub struct ProcessIndex<const N: usize> {
    by_name: [Option<(&'static str, usize)>; N],
}
