// Loads a pointed-to address at a label into a register
// and does the arithmetic to compute the actual address
// relative to a given base address from another register.
.macro LOAD_LABEL_ADDR register, base, symbol
    adr \register, \symbol
    ldr \register, [\register]
    add \register, \base, \register
.endm

.section .r0.text.start, "ax", %progbits
.global __saturnus_start
__saturnus_start:
    b __saturnus_loader_main

// The Saturnus KernelLoaderMeta structure. See the `kernel-image`
// crate for details. Make sure these two are always in sync.
__saturnus_loader_magic:
    .ascii "SLD0"
__saturnus_loader_version:
    .word 0xFFFFFFFF
__saturnus_loader_marker:
    .word 0xCCCCCCCC

// fn __saturnus_loader_main(
//     kernel_base: usize,
//     kernel_layout: *const KernelLayout,
//     ini1_base: usize,
// )
//
// This routine is responsible for doing the loader's own runtime
// initialization and then randomizing the mapping of the kernel
// sections in memory to employ KASLR.
//
// It returns some state it accumulates in the process of doing so
// that will be passed back to the Kernel upon returning.
//
.global __saturnus_loader_main
.type   __saturnus_loader_main, %function
__saturnus_loader_main:
    adr x18, __saturnus_start
    LOAD_LABEL_ADDR x16, x18, __saturnus_loader_bss_start
    LOAD_LABEL_ADDR x17, x18, __saturnus_loader_bss_end

    // Clear every byte pair in .bss section.
0:
    cmp x16, x17
    b.cs 1f
    stp xzr, xzr, [x16], #16
    b 0b

1:
    // Set the stack to the end of the initialized .bss section.
    LOAD_LABEL_ADDR x17, x18, __saturnus_loader_stack_top
    mov sp, x17

    // Back up our arguments and the link register on the stack.
    sub sp, sp, #0x20
    stp x0, x1, [sp, #0x00] // Store `kernel_base` and `kernel_map`.
    stp x2, lr, [sp, #0x10] // Store `ini1_base` and link register.

    // Apply all dynamic relocations to ourselves.
    adr x0, __saturnus_start
    LOAD_LABEL_ADDR x1, x0, __saturnus_loader_dynamic_start
    bl apply_relocations

    // TODO: Missing logic.

    mov x0, #0x18
    hlt #0xF000

.balign 8
__saturnus_loader_stack_top:
    .quad __stack_top__     - __saturnus_start
__saturnus_loader_bss_start:
    .quad __bss_start__     - __saturnus_start
__saturnus_loader_bss_end:
    .quad __bss_end__       - __saturnus_start
__saturnus_loader_dynamic_start:
    .quad _DYNAMIC          - __saturnus_start
__saturnus_loader_vectors_start:
    .quad __vectors_start__ - __saturnus_start
