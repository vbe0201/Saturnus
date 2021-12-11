//! Board Support Package to support miscellaneous platforms.

#[cfg(all(target_arch = "aarch64", feature = "bsp-nintendo-nx"))]
#[path = "bsp/aarch64/nintendo_nx.rs"]
mod bsp;

#[cfg(not(all(target_arch = "aarch64", feature = "bsp-nintendo-nx")))]
#[path = "bsp/generic.rs"]
mod bsp;

/// Generates random bytes.
///
/// This overrides the contents of the entire `buf` slice with the newly
/// generated data.
///
/// # Panics
///
/// Panics when `buf` exceeds a length of `0x38` bytes in total, which is
/// a conceptual limit for how many random bytes can be generated.
pub fn generate_random_bytes(buf: &mut [u8]) {
    bsp::generate_random_bytes(buf);
}

pub mod init {
    use super::bsp;

    /// Generates random bytes.
    ///
    /// This overrides the contents of the entire `buf` slice with the newly
    /// generated data.
    ///
    /// # Panics
    ///
    /// Panics when `buf` exceeds a length of `0x38` bytes in total, which is
    /// a conceptual limit for how many random bytes can be generated.
    pub unsafe fn generate_random_bytes(buf: &mut [u8]) {
        bsp::init::generate_random_bytes(buf);
    }
}
