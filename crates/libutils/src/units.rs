//! Helpers for working with different units of bytes.

/// 1 KB in bytes.
pub const KILOBYTE: u64 = 1_000;
/// 1 KiB in bytes.
pub const KIBIBYTE: u64 = 1 << 10;
/// 1 MB in bytes.
pub const MEGABYTE: u64 = 1_000_000;
/// 1 MiB in bytes.
pub const MEBIBYTE: u64 = 1 << 20;
/// 1 GB in bytes.
pub const GIGABYTE: u64 = 1_000_000_000;
/// 1 GiB in bytes.
pub const GIBIBYTE: u64 = 1 << 30;
/// 1 TB in bytes.
pub const TERABYTE: u64 = 1_000_000_000_000;
/// 1 TiB in bytes.
pub const TEBIBYTE: u64 = 1 << 40;
/// 1 PB in bytes.
pub const PETABYTE: u64 = 1_000_000_000_000_000;
/// 1 PiB in bytes.
pub const PEBIBYTE: u64 = 1 << 50;

/// Converts `n` KB to bytes.
#[inline(always)]
pub const fn kb(n: u64) -> u64 {
    n * KILOBYTE
}

/// Converts `n` KiB to bytes.
#[inline(always)]
pub const fn kib(n: u64) -> u64 {
    n * KIBIBYTE
}

/// Converts `n` MB to bytes.
#[inline(always)]
pub const fn mb(n: u64) -> u64 {
    n * MEGABYTE
}

/// Converts `n` MiB to bytes.
#[inline(always)]
pub const fn mib(n: u64) -> u64 {
    n * MEBIBYTE
}

/// Converts `n` GB to bytes.
#[inline(always)]
pub const fn gb(n: u64) -> u64 {
    n * GIGABYTE
}

/// Converts `n` GiB to bytes.
#[inline(always)]
pub const fn gib(n: u64) -> u64 {
    n * GIBIBYTE
}

/// Converts `n` TB to bytes.
#[inline(always)]
pub const fn tb(n: u64) -> u64 {
    n * TERABYTE
}

/// Converts `n` TiB to bytes.
#[inline(always)]
pub const fn tib(n: u64) -> u64 {
    n * TEBIBYTE
}

/// Converts `n` PB to bytes.
#[inline(always)]
pub const fn pb(n: u64) -> u64 {
    n * PETABYTE
}

/// Converts `n` PiB to bytes.
#[inline(always)]
pub const fn pib(n: u64) -> u64 {
    n * PEBIBYTE
}
