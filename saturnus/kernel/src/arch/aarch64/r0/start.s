// Loads a base memory address from a label into a register
// and does the arithmetic to compute the actual address
// relative to a given base from another register.
.macro LOAD_LABEL_ADDR register, base, symbol
    adr \register, \symbol
    ldr \register, [\register]
    add \register, \base, \register
.endm

// Global entrypoint to the Kernel. This is the start of
// every Kernel Image and the first bit of code executed.
.section .r0.text.start, "ax", %progbits
.global __saturnus_start
__saturnus_start:
    b bootstrap_kernel

// The Saturnus KernelMeta structure. See the `kernel-image`
// crate for details. Make sure these two are always in sync.
__saturnus_magic:
    .ascii "SKN0"
__saturnus_ini1_base:
    .quad 0x0000000000000000
__saturnus_kernel_loader_base:
    .quad 0x0000000000000000
__saturnus_version:
    .word 0xFFFFFFFF
__saturnus_kernel_layout:
    .word __saturnus_start - __saturnus_start  // text_start
    .word __text_end__     - __saturnus_start  // text_end
    .word __rodata_start__ - __saturnus_start  // rodata_start
    .word __rodata_end__   - __saturnus_start  // rodata_end
    .word __data_start__   - __saturnus_start  // data_start
    .word __data_end__     - __saturnus_start  // data_end
    .word __bss_start__    - __saturnus_start  // bss_start
    .word __bss_end__      - __saturnus_start  // bss_end
    .word __end__          - __saturnus_start  // kernel_end
    .word _DYNAMIC         - __saturnus_start  // dynamic_start

// fn __saturnus_bootstrap_kernel(...)
//
// This is responsible for bootstrapping the Kernel on the CPU
// core 0 we're initially executing on.
//
// It is responsible for ensuring the kernel runs under EL1,
// disabling the MMU and caches, executing the Kernel Loader to
// set up KASLR and finally invoke the actual kernel entrypoint.
//
.section .r0.text, "ax", %progbits
.global bootstrap_kernel
.type   bootstrap_kernel, %function
bootstrap_kernel:
    // Mask all interrupts.
    msr daifset, #0xF

    // Stash arguments away for later use.
    mov x19, x0
    mov x20, x1

    // Load the current EL we're executing under.
    mrs x1, currentel

    // Check if we're running under EL1.
    cmp x1, #0x4
    b.eq 1f

    // Check if we're running under EL2.
    cmp x1, #0x8
    b.eq 0f

    // We're running under EL3 at this point.
    // Let the responsible routine decide how to proceed.
    bl handle_running_under_el3

0:
    // We're currently running under EL2.
    // Let the responsible routine decide how to proceed.
    bl handle_running_under_el2

1:
    // We're running under EL1 now.

    // Disable the MMU and the instruction/data caches.
    bl flush_disable_mmu_and_caches

    // Compute the Kernel Loader entry point in memory and call it
    // with the following arguments:
    //
    //  - x0: The Kernel base address in memory.
    //  - x1: The Kernel layout map `__saturnus_kernel_layout`.
    //  - x2: The base address of the embedded INI1 resource.
    //
    // Loader returns its page allocator state in X0 for us to reuse.
    adr x0, __saturnus_start
    adr x1, __saturnus_kernel_layout
    LOAD_LABEL_ADDR x2, x0, __saturnus_ini1_base
    LOAD_LABEL_ADDR x3, x0, __saturnus_kernel_loader_base
    blr x3

// fn flush_disable_mmu_and_caches()
//
// Flushes the data cache, invalidates the instruction cache and
// disables both caches along with the Memory Management Unit.
//
// This will flush the data cache and invalidate the instruction
// cache before fully disabling both of them along with the MMU.
.section .r0.text, "ax", %progbits
.global flush_disable_mmu_and_caches
.type   flush_disable_mmu_and_caches, %function
flush_disable_mmu_and_caches:
    // Back up our link register in a callee-saved register.
    mov x22, lr

    // Flush the data cache and invalidate the entire TLB.
    bl flush_entire_data_cache_and_invalidate_tlb

    // Invalidate the caches and disable the MMU.
    bl disable_mmu_and_caches

    // Restore the saved link register and return to the caller.
    mov lr, x22
    ret
