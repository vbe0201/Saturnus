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
// TODO: Document this.
//
.section .r0.text, "ax", %progbits
.global __saturnus_loader_main
.type   __saturnus_loader_main, %function
__saturnus_loader_main:
    // Use QEMU semihosting to report success.
    adr x1, __test
    mov x0, #0x4
    hlt #0xF000

    // Halt QEMU, we're done.
    mov x0, #0x18
    hlt #0xF000

.balign 8
__test:
    .ascii "Works\n\0"
