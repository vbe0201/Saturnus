#![no_std]
#![no_main]
#![feature(asm, global_asm, naked_functions, option_get_or_insert_default)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate static_assertions;

mod exception;
mod panic;
mod reloc;

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
/// This is passed to [`main`] in order to relocate and randomize all the kernel mappings
/// in memory after enabling KASLR.
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
#[allow(unsafe_op_in_unsafe_fn)]
#[naked]
#[no_mangle]
unsafe extern "C" fn main(
    /* x0 */ _kernel_base: usize,
    /* x1 */ _kernel_map: *const KernelMap,
    /* x2 */ _ini1_base: usize,
) -> ! {
    asm!(
        r#"
        .macro REL_ADR register, symbol
            adrp \register, \symbol
            add  \register, \register, #:lo12:\symbol
        .endm

        REL_ADR x16, __bss_start__
        REL_ADR x17, __bss_end__

        // Clear every byte pair in the .bss segment.
    1:
        cmp x16, x17
        b.eq 2f
        stp xzr, xzr, [x16], #0x10
        b 1b

        // Point sp to the end of the .bss segment, where our stack begins.
    2:
        mov sp, x17

        // Back up our arguments and the link register on the stack.
        sub sp, sp, #0x20
        stp x0, x1,  [sp, #0x00] // Store `kernel_base` and `kernel_map`.
        stp x2, x30, [sp, #0x10] // Store `ini1_base` and link register.

        // Apply all dynamic relocations to ourselves.
        adr x0, _start
        REL_ADR x1, _DYNAMIC
        bl {apply_relocations}

        // Check if relocations were successful, otherwise loop infinitely
        cmp x0, xzr
        b.ne .

        // Run constructors in `.init_array` section
        bl {call_init_array}

        // Clear TPIDR_EL1 and set VBAR_EL1 to the exception vector table
        msr TPIDR_EL1, xzr

        // FIXME: `REL_ADDR` macro doesn't work with asm operand
        adrp x16, {exception_vector_table}
        add  x16, x16, #:lo12:{exception_vector_table}
        msr VBAR_EL1, x16

        // Exit QEMU using semihosting.
        mov x0, #0x18
        hlt #0xF000
    "#,
        apply_relocations = sym reloc::relocate,
        exception_vector_table = sym exception::EXCEPTION_TABLE,
        call_init_array = sym call_init_array,
        options(noreturn)
    )
}

/// Uniformly calls all the functions in the `.init_array` segment.
///
/// The `.init_array` functions of the program must be defined with
/// the [`init_array`] macro to get linked into the segment.
///
/// [`init_array`]: macro.init_array.html
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn call_init_array() {
    extern "C" {
        static __init_array_start__: unsafe extern "C" fn();
        static __init_array_end__: unsafe extern "C" fn();
    }

    // Calculate the amount of pointers that the .init_array segment holds.
    let init_array_length = (&__init_array_end__ as *const _ as usize
        - &__init_array_start__ as *const _ as usize)
        / mem::size_of::<unsafe extern "C" fn()>();

    // Compose a slice of all the function pointers in the segment and call them separately.
    for ptr in slice::from_raw_parts(&__init_array_start__, init_array_length) {
        ptr();
    }
}
