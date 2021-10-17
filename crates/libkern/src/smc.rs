//! Implementations of the Secure Monitor Calls featured by Horizon's Secure
//! Monitor which lives in EL3.

/// SMCs used throughout early kernel bootstrap.
pub mod init {

    /// Generates random bytes using the Secure Monitor's access to the Tegra
    /// Security Engine's CPRNG.
    pub fn generate_random_bytes<T>() -> Result<T, ()> {
        todo!()
    }
}
