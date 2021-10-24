//! Platform agnostic emulation of board specific functionality.

use core::mem::size_of;

use libutils::crc::crc32;

use crate::sync::SpinLock;

#[path = "generic/rand.rs"]
mod rand;

static RAND: SpinLock<rand::MtRand> = SpinLock::new(rand::MtRand::new(crc32(include_bytes!(
    "./generic/nvidia.png"
))));

pub mod init {
    // difference between `init` namespace and normal one is only present
    // on nintendo switch
    pub use super::generate_random_bytes;
}

pub fn generate_random_bytes(buf: &mut [u8]) {
    // emulate the same behaviour as the switch
    assert!(buf.len() <= size_of::<[u64; 8]>() - size_of::<u64>());

    RAND.lock().fill_bytes(buf)
}
