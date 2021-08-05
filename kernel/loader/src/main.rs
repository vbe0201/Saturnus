#![no_std]
#![no_main]
#![feature(asm, global_asm, naked_functions, option_get_or_insert_default)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate saturnus_semihosting;

#[macro_use]
extern crate static_assertions;

#[macro_use]
mod macros;

mod exception;
mod loader;
mod page_allocator;
mod panic;
mod rt;

use core::{mem, slice};
use loader::KernelMap;
use page_allocator::PageAllocator;

/// The global page allocator that is used throughout the loader's runtime
/// for allocating pages.
pub(crate) static INITAL_PAGE_ALLOCATOR: PageAllocator = PageAllocator::new();

// The program entrypoint which forwards execution as-is into [`main`].
global_asm!(
    r#"
    .section .text.r0, "ax", %progbits
    .global _start
    _start:
        b main
"#
);

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

        // Check if relocations were successful, otherwise loop infinitely.
        cmp x0, xzr
        b.ne .

        // Run constructors in `.init_array` section.
        bl {call_init_array}

        // Clear TPIDR_EL1 and set VBAR_EL1 to the exception vector table
        msr TPIDR_EL1, xzr
        REL_ADR x16, __vectors_start__
        msr VBAR_EL1, x16

        // Populate the global exception vector table.
        bl {setup_exception_table}

        // Load the kernel binary.
        ldp x0, x1,  [sp, #0x00] // Restore `kernel_base` and `kernel_map`.
        ldp x2, x30, [sp, #0x10] // Restore `ini1_base` and link register.
        add sp, sp, #0x20
        bl {load_kernel}

        // Exit QEMU using semihosting.
        mov x0, #0x18
        hlt #0xF000
    "#,
        apply_relocations = sym rt::relocate,
        call_init_array = sym rt::call_init_array,
        setup_exception_table = sym exception::setup_exception_vector,
        load_kernel = sym loader::load_kernel,
        options(noreturn)
    )
}
