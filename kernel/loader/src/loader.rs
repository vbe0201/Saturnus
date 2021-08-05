/// Address mappings of all relevant kernel segments in physical memory.
///
/// This is passed to [`main`] in order to relocate and randomize all the kernel mappings
/// in memory after enabling KASLR.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct KernelMap {
    /// The start offset of the kernel's `.text` segment.
    text_start: u32,
    /// The end offset of the kernel's `.text` segment.
    text_end: u32,
    /// The start offset of the kernel's `.rodata` segment.
    rodata_start: u32,
    /// The end offset of the kernel's `.rodata` segment.
    rodata_end: u32,
    /// The start offset of the kernel's `.data` segment.
    data_start: u32,
    /// The end offset of the kernel's `.data` segment.
    data_end: u32,
    /// The start offset of the kernel's `.bss` segment.
    bss_start: u32,
    /// The end offset of the kernel's `.bss` segment.
    bss_end: u32,
    /// The start offset of the kernel's `.ini1` segment.
    ini1: u32,
    /// The start offset of the kernel's `.dynamic` segment.
    dynamic: u32,
    /// The start offset of the kernel's `.init_array` segment.
    init_array_start: u32,
    /// The end offset of the kernel's `.init_array` segment.
    init_array_end: u32,
}

assert_eq_size!(KernelMap, [u8; 0x30]);

/// Relocates the kernel to a random base address, identity maps the kernel and prepares everything
/// for jumping back into the kernel.
///
/// # Returns
///
/// The relocated base address of the kernel.
pub unsafe extern "C" fn load_kernel(
    _kernel_base: usize,
    _kernel_map: *const KernelMap,
    _ini_base: usize,
) -> usize {
    todo!()
}

/// This retrieves memory layout information from the secure monitor,
/// and adjusts the kernel's physical location if necessary.
///
/// # Returns
///
/// The adjusted kernel base and kernel map pointer.
unsafe fn relocate_kernel_physically(
    kernel_base: usize,
    kernel_map: &KernelMap,
) -> (usize, &KernelMap) {
    // if the base address was adjusted, move the kernel to the new base
    // and return the new pointers
    if let Some(adjusted_base) = get_adjusted_kernel_base(kernel_base) {
        // copy kernel data to new location
        // FIXME: This can probably be adjusted to copy words instead of bytes
        unsafe {
            core::ptr::copy(
                kernel_base as *const u8,
                adjusted_base as *mut u8,
                kernel_map.data_end as usize,
            )
        };

        // calculate the new kernel base and kernel map pointer
        let diff = adjusted_base - kernel_base;
        let kernel_map = unsafe {
            let ptr = kernel_map as *const KernelMap as *const u8;
            let ptr = ptr.add(diff);
            &*ptr.cast::<KernelMap>()
        };

        (kernel_base + diff, kernel_map)
    } else {
        (kernel_base, kernel_map)
    }
}

/// This sees how much more memory is available than expected, and relocates the kernel accordingly.
///
/// # Returns
///
/// `None` if the kernel does not require any relocation, otherwise `Some` with the adjusted base
/// address.
#[allow(clippy::diverging_sub_expression)]
fn get_adjusted_kernel_base(base: usize) -> Option<usize> {
    // read DRAM size information from memory controller
    let dram_size_from_mc: usize = todo!("tegra210 implementation missing");

    // read DRAM size information from Secure Monitor KernelConfiguration
    let memory_type: usize = todo!("tegra210 implementation missing");

    // convert memory type to size of memory
    let dram_size_from_kernel_cfg = match memory_type {
        // MemoryType_6GB = 1
        1 => 6 << 30,
        // MemoryType_8GB = 2
        2 => 8 << 30,
        // MemoryType_4G  = 0 (default case)
        _ => 4 << 30,
    };

    // on normal systems, these should be equal and kernel will not be relocated
    if dram_size_from_mc < 2 * dram_size_from_kernel_cfg {
        Some(base + (dram_size_from_mc - dram_size_from_kernel_cfg) / 2)
    } else {
        None
    }
}
