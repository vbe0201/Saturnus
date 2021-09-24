//! Helper functions used throughout the crate.

/// Aligns value up to the next multiple of `align` and returns the result.
///
/// # Panics
///
/// This function panicks if `align` is not a power of `2`.
#[inline(always)]
pub const fn align_up(value: usize, align: usize) -> usize {
    align_down(value + align - 1, align)
}

/// Aligns `value` down to the next multiple of `align` and returns the result.
///
/// # Panics
///
/// This function panicks if `align` is not a power of `2`.
#[inline(always)]
pub const fn align_down(value: usize, align: usize) -> usize {
    assert!(align.is_power_of_two());
    value & !(align - 1)
}

/// Checks whether `value` is aligned to a multiple of `align`.
///
/// # Panics
///
/// This function panicks if `align` is not a power of `2`.
#[inline(always)]
pub const fn is_aligned(value: usize, align: usize) -> bool {
    assert!(align.is_power_of_two());
    value & (align - 1) == 0
}
