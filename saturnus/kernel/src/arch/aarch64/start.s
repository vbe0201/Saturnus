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
    b __saturnus_bootstrap_kernel

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
// TODO: Document this.
//
.section .r0.text, "ax", %progbits
.global __saturnus_bootstrap_kernel
.type   __saturnus_bootstrap_kernel, %function
__saturnus_bootstrap_kernel:
    // Compute the Kernel Loader entry point in memory and call it
    // with the following arguments:
    //
    //  - x0: The Kernel base address in memory.
    //  - x1: The Kernel layout map `__saturnus_kernel_layout`.
    //  - x2: The base address of the embedded INI1 resource.
    adr x0, __saturnus_start
    adr x1, __saturnus_kernel_layout
    LOAD_LABEL_ADDR x2, x0, __saturnus_ini1_base
    LOAD_LABEL_ADDR x3, x0, __saturnus_kernel_loader_base
    blr x3
