//! Board Support Package to support miscellaneous platforms of the same architecture.

#[cfg(all(target_arch = "aarch64", feature = "bsp-nintendo-nx"))]
#[path = "aarch64/nintendo/nx/resources.rs"]
mod bsp_resources;

#[cfg(all(target_arch = "aarch64", feature = "bsp-qemu"))]
#[path = "aarch64/qemu/resources.rs"]
mod bsp_resources;

/// Takes the physical kernel base address and determines from available memory size
/// whether a physical relocation to higher addresses should take place.
///
/// Based on the outcome, the function either returns `Some` if the base was adjusted
/// to a more fitting address, or `None` if the current `base` should remain.
pub fn adjust_kernel_base(base: usize) -> Option<usize> {
    bsp_resources::adjust_kernel_base(base)
}

/// This function is called when the kernel is loaded and determines if the loader
/// will reserve an [additional](crate::loader::ADDITIONAL_KERNEL_DATA_SIZE) amount
/// of data to be used by the kernel.
pub fn reserve_additional_kernel_data() -> bool {
    bsp_resources::reserve_additional_kernel_data()
}
