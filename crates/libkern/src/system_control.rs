//! Machinery for querying and modifying system state related to the execution
//! environment.

/// System control APIs for early kernel bootstrap.
pub mod init {
    use core::mem;

    use crate::bsp;

    /// Generates a random range between a minimal and a maximal bound.
    ///
    /// This method does not use a cryptographically secure number generator!
    pub unsafe fn generate_random_range(min: usize, max: usize) -> usize {
        let mut bytes = [0; mem::size_of::<usize>()];
        bsp::init::generate_random_bytes(&mut bytes);

        let range_size = max - min + 1;
        min + usize::from_ne_bytes(bytes) % range_size
    }
}
