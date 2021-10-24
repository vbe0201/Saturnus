//! Helpers for interpreting memory.

use core::mem;

/// Returns the size of a given type in bits.
///
/// See [`std::mem::size_of`] for semantic information on how sizes in bytes are determined.
/// This function merely converts such a value in bytes to bits.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::mem::size_of;
///
/// use saturnus_libutils::mem::bit_size_of;
///
/// assert_eq!(size_of::<u32>(), 4);
/// assert_eq!(bit_size_of::<u32>(), 32);
/// ```
#[inline]
pub const fn bit_size_of<T>() -> usize {
    mem::size_of::<T>() << 3
}

/// Returns the size of the pointed-to value in bits.
///
/// See [`std::mem::size_of_val`] for semantic information on how sizes in bytes are
/// determined. This function merely converts such a value in bytes to bits.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::mem::size_of_val;
///
/// use saturnus_libutils::mem::bit_size_of_val;
///
/// let x: u32 = 5;
///
/// assert_eq!(size_of_val(&x), 4);
/// assert_eq!(bit_size_of_val(&x), 32);
/// ```
#[inline]
pub fn bit_size_of_val<T: ?Sized>(val: &T) -> usize {
    mem::size_of_val(val) << 3
}

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

/// Returns the value of type `T` represented by the all-zero byte pattern.
///
/// # Safety
///
/// The Rust compiler assumes that every initialized variable contains a
/// valid value. Thereby this function will cause immediate undefined behavior
/// if a sequence of null bytes is not a valid representation of `T`. Examples
/// being function pointers or reference types.
///
/// # Examples
///
/// ```
/// use saturnus_libutils::mem::zeroed;
///
/// // SAFETY: A byte pattern of zeroes is valid for every integral type and
/// // we're guaranteed that array elements reside subsequently in memory.
/// const NULL_ARRAY: [u32; 5] = unsafe { zeroed() };
///
/// assert_eq!(NULL_ARRAY[4], 0);
/// ```
#[inline(always)]
pub const unsafe fn zeroed<T>() -> T
where
    [(); mem::size_of::<T>()]: Sized,
{
    let zeroed = [0u8; mem::size_of::<T>()];
    unsafe { mem::transmute_copy(&zeroed) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_up() {
        assert_eq!(align_up(2, 8), 8);
        assert_eq!(align_up(31, 16), 32);
        assert_eq!(align_up(32, 32), 32);
        assert_eq!(align_up(0, 16), 0);
    }

    #[test]
    fn test_align_down() {
        assert_eq!(align_down(44, 8), 40);
        assert_eq!(align_down(44, 16), 32);
        assert_eq!(align_down(8, 8), 8);
        assert_eq!(align_down(0, 256), 0);
    }

    #[test]
    fn test_is_aligned() {
        assert!(is_aligned(0, 8));
        assert!(is_aligned(8, 8));
        assert!(is_aligned(64, 16));
    }

    #[test]
    #[should_panic]
    fn test_invalid_align_up() {
        align_up(2, 5);
    }

    #[test]
    #[should_panic]
    fn test_invalid_align_down() {
        align_down(40, 3);
    }
}
