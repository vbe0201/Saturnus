//! Helpers for working with bit sizes and bit extractions.

use core::mem;

/// Converts a given value in bits to bytes.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use saturnus_libutils::bits::bits_to_bytes;
/// assert_eq!(bits_to_bytes(32), 4);
/// ```
#[inline(always)]
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
/// # use saturnus_libutils::bits::bytes_to_bits;
/// assert_eq!(bytes_to_bits(4), 32);
/// ```
#[inline(always)]
pub const fn bytes_to_bits(nbytes: usize) -> usize {
    nbytes << 3
}

/// Returns the size of a type in bits.
///
/// This uses [`core::mem::size_of`] to determine the size in bytes
/// and translates that value to bits.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use saturnus_libutils::bits::bit_size_of;
/// assert_eq!(bit_size_of::<u32>(), 32);
/// ```
#[inline(always)]
pub const fn bit_size_of<T>() -> usize {
    bytes_to_bits(mem::size_of::<T>())
}

/// Crafts a bitmask that represents the `n` least significant bits.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use saturnus_libutils::bits::least_significant_bits;
/// assert_eq!(least_significant_bits(3), 0b111);
/// ```
#[inline(always)]
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
/// # use saturnus_libutils::bits::bitmask;
/// assert_eq!(bitmask(2, 6), 0b111100)
/// ```
#[inline(always)]
pub const fn bitmask(start: usize, end: usize) -> usize {
    least_significant_bits(end) & !least_significant_bits(start)
}
