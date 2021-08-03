#![no_std]
#![no_main]
#![feature(asm, global_asm, naked_functions)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate static_assertions;

mod panic;

// The program entrypoint which forwards execution as-is into [`main`].
global_asm!(
    r#"
    .section .text.r0, "ax", %progbits
    .global _start
    _start:
        b main
"#
);

/// Address mappings of all relevant kernel segments in physical memory.
///
/// This is passed to [`main`] in order to relocate and randomize all the kernel segments
/// after enabling KASLR.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct KernelMap {
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

/// The entrypoint to the Kernel Loader application.
///
/// This is called by the Kernel's r0 to enable KASLR, apply kernel relocations and randomize the
/// memory mapping of all kernel segments prior to handing execution back to the kernel itself.
#[naked]
#[no_mangle]
unsafe extern "C" fn main(kernel_base: usize, kernel_map: *const KernelMap, ini1_base: usize) -> ! {
    // TODO
    loop {}
}
