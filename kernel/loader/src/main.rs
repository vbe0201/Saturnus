#![no_std]
#![no_main]
#![feature(asm, naked_functions, option_get_or_insert_default)]
#![deny(rustdoc::broken_intra_doc_links, unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate static_assertions;

mod panic;
mod reloc;

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

// The program entrypoint which forwards execution as-is into [`r0::main`].
#[naked]
#[no_mangle]
#[link_section = ".text.r0"]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn _start(
    _kernel_base: usize,
    _kernel_map: *const KernelMap,
    _ini1_base: usize,
) -> ! {
    asm!(
        "b {}",
        sym main,
        options(noreturn)
    )
}

/// The entrypoint to the Kernel Loader application.
///
/// This is called by the Kernel's r0 to enable KASLR, apply kernel relocations and randomize the
/// memory mapping of all kernel segments prior to handing execution back to the kernel itself.
#[naked]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn main(
    _kernel_base: usize,
    _kernel_map: *const KernelMap,
    _ini1_base: usize,
) -> ! {
    asm!(
        "
        // Load bss start and end addresses into `x3` and `x4`
        ldr x3, =__bss_start__
        ldr x4, =__bss_end__

        // Loop over every byte pair in the bss section and write 0 to it.
    1:
        cmp x3, x4
        b.eq 2f
        stp xzr, xzr, [x3], #16
        b 1b

        // Load stack pointer, which is end of bss section, because that's
        // where our stack lives (see `linker-scripts/loader.ld`)
    2:
        mov sp, x3

        // Relocate the loader by calling `reloc::relocate` with our base address,
        // and the start of the `.dynamic` section
        ldr x0, =__start__
        ldr x1, =__dynamic_start__
        bl {reloc}

        // Exit QEMU using semihosting
    3:  mov x0, #0x18
        hlt #0xF000",
        reloc = sym reloc::relocate,
        options(noreturn)
    )
}
