//! Helpers for aligning memory addresses.

/// Aligns `value` up to the next multiple of `align`.
///
/// # Panics
///
/// Panics when `align` is not a power of two.
#[inline(always)]
pub const fn align_up(value: usize, align: usize) -> usize {
    align_down(value + align - 1, align)
}

/// Aligns `value` down to the next multiple of `align`.
///
/// # Panics
///
/// Panics when `align` is not a power of two.
#[inline(always)]
pub const fn align_down(value: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two());
    value & !(align - 1)
}

/// Checks whether `value` is aligned to a multiple of `align`.
///
/// # Panics
///
/// Panics when `align` is not a power of two.
#[inline(always)]
pub const fn is_aligned(value: usize, align: usize) -> bool {
    debug_assert!(align.is_power_of_two());
    value & (align - 1) == 0
}

/// Finds the smallest alignment to a power of two for `value`.
#[inline(always)]
pub const fn align_of(value: usize) -> usize {
    let value = value as isize;
    (value & -value) as usize
}
