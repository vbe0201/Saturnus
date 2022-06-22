#![feature(const_option, naked_functions, strict_provenance)]
#![no_std]
#![no_main]

use core::{mem::size_of, ptr};

use utils::align::is_aligned;

mod arch;
mod board;
mod panic;
mod reloc;

const BUILD_CONFIG: config::Config = config::CURRENT_BUILD.unwrap();

/// The layout of Kernel binary sections in memory.
///
/// This is supplied by the Kernel itself when it calls Loader.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct KernelLayout {
    /// The start offset of the kernel's `.text` section.
    text_start: u32,
    /// The end offset of the kernel's `.text` section.
    text_end: u32,
    /// The start offset of the kernel's `.rodata` section.
    rodata_start: u32,
    /// The end offset of the kernel's `.rodata` section.
    rodata_end: u32,
    /// The start offset of the kernel's `.data` section.
    data_start: u32,
    /// The end offset of the kernel's `.data` section.
    data_end: u32,
    /// The start offset of the kernel's `.bss` section.
    bss_start: u32,
    /// The end offset of the kernel's `.bss` section.
    bss_end: u32,
    /// The end offset of the kernel binary.
    kernel_end: u32,
    /// The start offset of the kernel's `_DYNAMIC` array.
    dynamic_start: u32,
}

const _: () = assert!(size_of::<KernelLayout>() == 0x28);

#[no_mangle]
unsafe extern "C" fn main(
    kernel_base: *mut u8,
    kernel_layout: *const KernelLayout,
    ini1_base: *const u8,
) -> *const u8 {
    // Relocate the Kernel in physical memory, if necessary.
    let (kernel_base, kernel_layout) = relocate_kernel_physically(kernel_base, kernel_layout);
    let kernel_layout = &*kernel_layout;

    // Validate the kernel layout.
    assert!(is_aligned(
        kernel_layout.text_start as _,
        BUILD_CONFIG.page_size
    ));
    assert!(is_aligned(
        kernel_layout.text_end as _,
        BUILD_CONFIG.page_size
    ));
    assert!(is_aligned(
        kernel_layout.rodata_start as _,
        BUILD_CONFIG.page_size
    ));
    assert!(is_aligned(
        kernel_layout.rodata_end as _,
        BUILD_CONFIG.page_size
    ));
    assert!(is_aligned(
        kernel_layout.data_start as _,
        BUILD_CONFIG.page_size
    ));
    assert!(is_aligned(
        kernel_layout.bss_end as _,
        BUILD_CONFIG.page_size
    ));

    todo!()
}

/// Performs physical relocation of the Kernel in memory.
///
/// # Safety
///
/// - `kernel_base` must be non-zero, well-aligned and a pointer
///   to the actual base of the Kernel in physical memory.
///
/// - `kernel_layout` must be non-zero, well-aligned and a pointer
///   to an actual [`KernelLayout`] allocation in memory.
unsafe fn relocate_kernel_physically(
    kernel_base: *mut u8,
    kernel_layout: *const KernelLayout,
) -> (*mut u8, *const KernelLayout) {
    match board::system_control::adjust_kernel_base(kernel_base) {
        Some(adjusted_kernel_base) => {
            // Move the kernel to the newly determined base in memory.
            // SAFETY: MMU is disabled; accessing physical memory is safe.
            // The other contracts are upheld by the caller.
            ptr::copy(
                kernel_base,
                adjusted_kernel_base,
                (*kernel_layout).data_end as usize,
            );

            // Now that things moved in memory, we need to adjust the pointers.
            let diff = adjusted_kernel_base.addr() - kernel_base.addr();
            (
                kernel_base.add(diff),
                kernel_layout.cast::<u8>().add(diff).cast::<KernelLayout>(),
            )
        }

        None => (kernel_base, kernel_layout),
    }
}
