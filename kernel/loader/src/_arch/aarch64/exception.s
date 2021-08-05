/*
 * Copyright (c) 2013-2017, ARM Limited and Contributors. All rights reserved.
 * Copyright (c) 2021, Valentin B.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

// https://github.com/ARM-software/arm-trusted-firmware/blob/master/include/arch/aarch64/asm_macros.S

/*
 * Declare the exception vector table, enforcing it is aligned on a
 * 2KB boundary, as required by the ARMv8 architecture.
 * Use zero bytes as the fill value to be stored in the padding bytes
 * so that it inserts illegal AArch64 instructions. This increases
 * security, robustness and potentially facilitates debugging.
 */
.macro VECTOR_BASE label, section_name=.vectors
.section \section_name, "ax"
.align 11, 0
\label:
.endm

/*
 * Create an entry in the exception vector table, enforcing it is
 * aligned on a 128-byte boundary, as required by the ARMv8 architecture.
 * Use zero bytes as the fill value to be stored in the padding bytes
 * so that it inserts illegal AArch64 instructions. This increases
 * security, robustness and potentially facilitates debugging.
 */
.macro VECTOR_ENTRY label, section_name=.vectors
.cfi_sections .debug_frame
.section \section_name, "ax"
.align 7, 0
.type \label, %function
.cfi_startproc
\label:
.endm

/*
 * This macro should be inserted at the end of an exception vector definition
 * and is responsible for padding its size to the limit of 32 instructions.
 * In case the exception vector has already exceeded the limit of 32 instructions
 * at the point of this macro invocation, an assembler error is emitted.
 */
.macro END_VECTOR_ENTRY since
.cfi_endproc
//.if (. - \since) > (32 * 4)
//    .error "Exception vector exceeds the limit of 32 instructions"
//.endif
.fill \since + (32 * 4) - .
.endm

/*
 * Invokes a given exception handler using a Rust `ExceptionContext` object
 * which holds important system state.
 */
.macro DISPATCH_WITH_CONTEXT handler
    // Reserve stack memory for the exception context.
    sub sp, sp, #8 * 33

    // Initialize `ExceptionContext::gpr`.
    stp x0,  x1,  [sp, #8 * 0]
    stp x2,  x3,  [sp, #8 * 2]
    stp x4,  x5,  [sp, #8 * 4]
    stp x6,  x7,  [sp, #8 * 6]
    stp x8,  x9,  [sp, #8 * 8]
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

    // Initialize `ExceptionContext::lr` and `ExceptionContext::elr_el1`.
    mrs x16, ELR_EL1
    stp lr,  x16, [sp, #8 * 30]

    // Initialize `ExceptionContext::spsr_el1`.
    mrs x17, SPSR_EL1
    str x17,      [sp, #8 * 32]

    // Call the exception handler using the allocated `ExceptionContext`.
    mov x0, sp
    bl \handler

    // Leave the exception handler and restore back to regular execution.
    b __exception_restore_context
.endm

/*
 * Suspends the current core in response to an incoming FIQ.
 */
.macro FIQ_SUSPEND
1:
    wfe
    b 1b
.endm

/*
 * Definitions of the actual exception vectors.
 */

.global __exception_vector_table
VECTOR_BASE __exception_vector_table

// Current exception level with SP_EL0.

VECTOR_ENTRY current_el0_synchronous
    DISPATCH_WITH_CONTEXT unsupported_exception_handler
END_VECTOR_ENTRY current_el0_synchronous

VECTOR_ENTRY current_el0_irq
    DISPATCH_WITH_CONTEXT unsupported_exception_handler
END_VECTOR_ENTRY current_el0_irq

VECTOR_ENTRY current_el0_fiq
    FIQ_SUSPEND
END_VECTOR_ENTRY current_el0_fiq

VECTOR_ENTRY current_el0_serror
    DISPATCH_WITH_CONTEXT unsupported_exception_handler
END_VECTOR_ENTRY current_el0_serror

// Current exception level with SP_ELx where x > 0.

VECTOR_ENTRY current_elx_synchronous
    DISPATCH_WITH_CONTEXT default_exception_handler
END_VECTOR_ENTRY current_elx_synchronous

VECTOR_ENTRY current_elx_irq
    DISPATCH_WITH_CONTEXT default_exception_handler
END_VECTOR_ENTRY current_elx_irq

VECTOR_ENTRY current_elx_fiq
    FIQ_SUSPEND
END_VECTOR_ENTRY current_elx_fiq

VECTOR_ENTRY current_elx_serror
    DISPATCH_WITH_CONTEXT default_exception_handler
END_VECTOR_ENTRY current_elx_serror

// Lower exception level, AArch64.

VECTOR_ENTRY lower_aarch64_synchronous
    DISPATCH_WITH_CONTEXT default_exception_handler
END_VECTOR_ENTRY lower_aarch64_synchronous

VECTOR_ENTRY lower_aarch64_irq
    DISPATCH_WITH_CONTEXT default_exception_handler
END_VECTOR_ENTRY lower_aarch64_irq

VECTOR_ENTRY lower_aarch64_fiq
    FIQ_SUSPEND
END_VECTOR_ENTRY lower_aarch64_fiq

VECTOR_ENTRY lower_aarch64_serror
    DISPATCH_WITH_CONTEXT default_exception_handler
END_VECTOR_ENTRY lower_aarch64_serror

// Lower exception level, AArch32.

VECTOR_ENTRY lower_aarch32_synchronous
    DISPATCH_WITH_CONTEXT default_exception_handler
END_VECTOR_ENTRY lower_aarch32_synchronous

VECTOR_ENTRY lower_aarch32_irq
    DISPATCH_WITH_CONTEXT default_exception_handler
END_VECTOR_ENTRY lower_aarch32_irq

VECTOR_ENTRY lower_aarch32_fiq
    FIQ_SUSPEND
END_VECTOR_ENTRY lower_aarch32_fiq

VECTOR_ENTRY lower_aarch32_serror
    DISPATCH_WITH_CONTEXT default_exception_handler
END_VECTOR_ENTRY lower_aarch32_serror

.type __exception_restore_context, %function
__exception_restore_context:
    // Restore `ExceptionContext::spsr_el1` to system state.
    ldr x19, [sp, #8 * 32]
    msr SPSR_EL1, x19

    // Restore `ExceptionContext::lr` and `ExceptionContext::elr_el1` to system state.
    ldp lr, x20, [sp, #8 * 30]
    msr ELR_EL1, x20

    // Restore `ExceptionContext::gpr` to system state.
    ldp x0,  x1,  [sp, #8 * 0]
    ldp x2,  x3,  [sp, #8 * 2]
    ldp x4,  x5,  [sp, #8 * 4]
    ldp x6,  x7,  [sp, #8 * 6]
    ldp x8,  x9,  [sp, #8 * 8]
    ldp x10, x11, [sp, #8 * 10]
    ldp x12, x13, [sp, #8 * 12]
    ldp x14, x15, [sp, #8 * 14]
    ldp x16, x17, [sp, #8 * 16]
    ldp x18, x19, [sp, #8 * 18]
    ldp x20, x21, [sp, #8 * 20]
    ldp x22, x23, [sp, #8 * 22]
    ldp x24, x25, [sp, #8 * 24]
    ldp x26, x27, [sp, #8 * 26]
    ldp x28, x29, [sp, #8 * 28]

    // Deallocate the `ExceptionContext` object on the stack.
    add sp, sp, #8 * 33

    // Return from exception context.
    eret

.size __exception_restore_context, . - __exception_restore_context
