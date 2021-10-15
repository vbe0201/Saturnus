//! Helper functions used throughout the crate.

/// Converts a given value in bits to bytes.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use kronos_utils::bits::bits_to_bytes;
///
/// assert_eq!(bits_to_bytes(32), 4);
/// ```
#[inline]
pub const fn bits_to_bytes(nbits: usize) -> usize {
    nbits >> 3
}

/// Converts a given value in bytes to bits.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use kronos_utils::bits::bytes_to_bits;
///
/// assert_eq!(bytes_to_bits(4), 32);
/// ```
#[inline]
pub const fn bytes_to_bits(nbytes: usize) -> usize {
    nbytes << 3
}

/// Crafts a bitmask that represents the `n` least significant bits.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use kronos_utils::bits::least_significant_bits;
///
/// assert_eq!(least_significant_bits(3), 0b111);
/// ```
#[inline]
pub const fn least_significant_bits(n: usize) -> usize {
    (1 << n) - 1
}

/// Crafts a bitmask that extracts a given bit range from `start` (inclusive) to
/// `end` (exclusive).
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use kronos_utils::bits::bitmask;
///
/// assert_eq!(bitmask(2, 6), 0b111100)
/// ```
#[inline]
pub const fn bitmask(start: usize, end: usize) -> usize {
    least_significant_bits(end) & !least_significant_bits(start)
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
