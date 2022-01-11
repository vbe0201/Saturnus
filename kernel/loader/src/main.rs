#![no_std]
#![no_main]
#![feature(asm_sym, naked_functions, option_get_or_insert_default)]
#![deny(unsafe_op_in_unsafe_fn, rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate semihosting;

#[macro_use]
extern crate static_assertions;

#[macro_use]
mod macros;

pub mod bsp;
pub mod exception;
pub mod loader;
pub mod page_allocator;
pub mod paging;
pub mod panic;
pub mod rt;

use page_allocator::PageAllocator;

use crate::loader::KernelMap;

// Source linker entrypoint from assembly.
::core::arch::global_asm!(
    r#"
    .section .text.r0, "ax", %progbits
    .global _start
    _start:
        // Forward execution as-is into main.
        b main

    .size _start, . - _start
"#
);

/// The global page allocator that is used throughout the loader's runtime
/// for allocating pages.
pub(crate) static INITAL_PAGE_ALLOCATOR: PageAllocator = PageAllocator::new();

/// The main function of the kernel loader, which is called by the kernel's `r0`.
///
/// It is responsible for setting up the loader's execution environment, enabling
/// KASLR and randomizing the kernel mappings in memory before yielding execution
/// back to the kernel itself.
#[allow(unsafe_op_in_unsafe_fn)]
#[naked]
#[no_mangle]
pub unsafe extern "C" fn main(
    /* x0 */ _kernel_base: usize,
    /* x1 */ _kernel_map: *const KernelMap,
    /* x2 */ _ini1_base: usize,
) -> ! {
    ::core::arch::asm!(
        r#"
        .macro REL_ADR register, symbol
            adrp \register, \symbol
            add \register, \register, #:lo12:\symbol
        .endm

        REL_ADR x16, __bss_start__
        REL_ADR x17, __bss_end__

        // Clear every byte pair in the .bss segment.
    1:
        cmp x16, x17
        b.eq 2f
        stp xzr, xzr, [x16], #16
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

        // Check if the operation were successful, otherwise loop infinitely.
        cmp x0, xzr
        b.ne .

        // Run constructors in `.init_array` section.
        bl {call_init_array}

        // Setup exception handling for catching runtime errors.
        msr TPIDR_EL1, xzr
        bl {setup_exception_vector}

        // Load the kernel segments and map them at randomized locations.
        ldp x0, x1,  [sp, #0x00] // Restore `kernel_base` and `kernel_map`.
        ldr x2,      [sp, #0x10] // Restore `ini1_base`.
        bl {load_kernel}

        // Exit QEMU using semihosting.
        mov x0, #0x18
        hlt #0xF000
    "#,
        apply_relocations = sym rt::relocate,
        call_init_array = sym rt::call_init_array,
        setup_exception_vector = sym exception::setup_exception_vector,
        load_kernel = sym loader::load_kernel,
        options(noreturn)
    )
}
