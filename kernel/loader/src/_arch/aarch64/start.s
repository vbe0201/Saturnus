//
// Implementation of the linker entrypoint to the loader application.
//

.section .text.r0, "ax", %progbits
.global _start
_start:
    // Forward execution as-is into main.
    b main

.size _start, . - _start
