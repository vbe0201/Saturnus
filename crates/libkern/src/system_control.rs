//! Machinery for querying and modifying system state related to the execution
//! environment.

/// System control APIs for early kernel bootstrap.
pub mod init {
    use core::mem;

    use crate::smc;

    /// Generates a random range between a minimal and a maximal bound.
    ///
    /// This uses a cryptographically secure pseudo-random number generator
    /// implemented in the EL3 Secure Monitor.
    pub fn generate_random_range(min: usize, max: usize) -> usize {
        let mut bytes = [0; mem::size_of::<usize>()];
        smc::init::generate_random_bytes(&mut bytes);

        usize::from_ne_bytes(bytes) % (max - min + 1) + min
    }
}
