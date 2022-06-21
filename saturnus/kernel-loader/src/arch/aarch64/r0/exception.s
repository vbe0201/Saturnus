// https://github.com/ARM-software/arm-trusted-firmware/blob/master/include/arch/aarch64/asm_macros.S

// Declares the exception vector table, enforcing it is aligned
// to 2KB boundary, as required by the ARMv8 architecture.
// Emit null bytes as padding which make up illegal AArch64
// instructions for increased security and robustness.
.macro VECTOR_BASE label, section_name=.vectors
    .section \section_name, "ax"
    .align 11, 0
    \label:
.endm

// Creates an entry in the exception vector table, enforcing it
// is aligned to 128-byte boundary, as required by the ARMv8
// architecture. Emit null bytes as padding which make up illegal
// AArch64 instructions for increased security and robustness.
.macro VECTOR_ENTRY label, section_name=.vectors
    .cfi_sections .debug_frame
    .section \section_name, "ax"
    .align 7, 0
    .type \label, %function
    .cfi_startproc
    \label:
.endm

// Marks the end of an exception vector entry definition and
// pads its size to the limit of 32 CPU instructions.
// In case the definition has already exceeded that limit at
// the point of macro invocation, an assembler error is produced.
.macro END_VECTOR_ENTRY since
    .cfi_endproc
    .fill \since + (32 * 4) - .
.endm

// Invokes a given exception handler routine with a reference to
// the current `ExceptionContext` object.
.macro DISPATCH_WITH_CONTEXT handler
    // Reserve stack memory for the exception context.
    sub sp, sp, #8 * 33

    // Initialize `ExceptionContext::gpr`.
    stp x0,  x1,  [sp, #8 *  0]
    stp x2,  x3,  [sp, #8 *  2]
    stp x4,  x5,  [sp, #8 *  4]
    stp x6,  x7,  [sp, #8 *  6]
    stp x8,  x9,  [sp, #8 *  8]
    stp x10, x11, [sp, #8 * 10]
    stp x12, x13, [sp, #8 * 12]
    stp x14, x15, [sp, #8 * 14]
    stp x16, x17, [sp, #8 * 16]
    stp x18, x19, [sp, #8 * 18]
    stp x20, x21, [sp, #8 * 20]
    stp x22, x23, [sp, #8 * 22]
    stp x24, x25, [sp, #8 * 24]
    stp x26, x27, [sp, #8 * 26]
    stp x28, x29, [sp, #8 * 28]

    // Initialize `ExceptionContext::lr` and `ExceptionContext::elr`.
    mrs x16, elr_el1
    stp lr, x16, [sp, #8 * 30]

    // Initialize `ExceptionContext::spsr`.
    mrs x17, spsr_el1
    str x17, [sp, #8 * 32]

    // Call the exception handler using the allocated `ExceptionContext`.
    // NOTE: We're not expecting `handler` to return.
    mov x0, sp
    b \handler
.endm

.global __vector_table__
VECTOR_BASE __vector_table__

// Current exception level with SP_EL0.

VECTOR_ENTRY current_el0_synchronous
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY current_el0_synchronous

VECTOR_ENTRY current_el0_irq
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY current_el0_irq

VECTOR_ENTRY current_el0_fiq
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY current_el0_fiq

VECTOR_ENTRY current_el0_serror
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY current_el0_serror

// Current exception level with SP_ELx where x > 0.

VECTOR_ENTRY current_elx_synchronous
    // TODO: Implement properly.
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY current_elx_synchronous

VECTOR_ENTRY current_elx_irq
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY current_elx_irq

VECTOR_ENTRY current_elx_fiq
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY current_elx_fiq

VECTOR_ENTRY current_elx_serror
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY current_elx_serror

// Lower exception level, AArch64.

VECTOR_ENTRY lower_aarch64_synchronous
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY lower_aarch64_synchronous

VECTOR_ENTRY lower_aarch64_irq
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY lower_aarch64_irq

VECTOR_ENTRY lower_aarch64_fiq
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY lower_aarch64_fiq

VECTOR_ENTRY lower_aarch64_serror
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY lower_aarch64_serror

// Lower exception level, AArch32.

VECTOR_ENTRY lower_aarch32_synchronous
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY lower_aarch32_synchronous

VECTOR_ENTRY lower_aarch32_irq
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY lower_aarch32_irq

VECTOR_ENTRY lower_aarch32_fiq
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY lower_aarch32_fiq

VECTOR_ENTRY lower_aarch32_serror
    DISPATCH_WITH_CONTEXT undefined_exception_handler
END_VECTOR_ENTRY lower_aarch32_serror
