//! Machinery for the early kernel initialization work.

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/init/mod.rs"]
mod arch_init;

/// Implementations of page allocation facilities to be used during early
/// kernel bootstrap.
///
/// At later stages of the system, a different implementation of paging
/// functionality should be used.
pub mod paging {
    use super::arch_init;

    pub use arch_init::paging::{InitialPageAllocator, InitialPageTable};
}
