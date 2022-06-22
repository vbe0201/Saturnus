//! Support for specific boards which require target-specific
//! implementation details.

#[cfg(feature = "qemu")]
#[path = "qemu/mod.rs"]
mod bsp_impl;

/// Target system control and configuration.
pub mod system_control {
    use super::bsp_impl;

    /// Takes the physical Kernel base and determines from target
    /// heuristics whether a physical relocation can happen.
    ///
    /// Based on the decision, this function either returns [`Some`]
    /// with the newly determined base pointer, or [`None`].
    pub fn adjust_kernel_base(base: *mut u8) -> Option<*mut u8> {
        bsp_impl::system_control::adjust_kernel_base(base)
    }
}
