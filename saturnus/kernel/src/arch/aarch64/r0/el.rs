//! Exception Level handlers for early Kernel bootstrap.
//!
//! Generally speaking, the Saturnus Kernel is expected to be
//! executing under EL1. For that reason we provide the
//! [`handle_running_under_el2`] and [`handle_running_under_el3`]
//! functions which are responsible for defining target-specific
//! behavior to apply when execution is not running under EL1.
//!
//! In other words, we either want to get the Kernel to proceed
//! in EL1 or not to proceed at all. Implementations should
//! respect this design goal and API users should build around it.
//!
//! The implementations are stackless so they can be used in
//! early bootstrap context.

use core::arch::asm;

use cortex_a::registers::{ACTLR_EL2, ELR_EL2, HCR_EL2, MIDR_EL1, SPSR_EL2};
use tock_registers::interfaces::{Readable, Writeable};

use super::cache::flush_entire_data_cache_and_invalidate_tlb;

const PARTNUM_CORTEX_A53: u64 = 0xD03;
const PARTNUM_CORTEX_A57: u64 = 0xD07;

/// Handles the execution of the Kernel under EL2.
///
/// Typically, implementations either deprivilege to EL1 or panic
/// when EL2 execution in the first place is not allowed.
///
/// # Note
///
/// This function does not make use of the stack.
///
/// # Safety
///
/// This is hardware land. Use cautiously.
#[cfg(feature = "qemu")]
#[naked]
#[no_mangle]
pub unsafe extern "C" fn handle_running_under_el2() -> ! {
    asm!(
        r#"
        // Back up the current link register in a callee-saved register.
        mov x24, lr

        // Flush the data cache and invalidate the entire TLB.
        bl {flush_entire_data_cache_and_invalidate_tlb}

        // Prepare system registers for deprivileging to EL1.
        // We want to jump to this function's return address.
        mov x1, x24
        bl {prepare_el2_to_el1_transition}

        // Lastly, return back to the caller under EL1.
        eret
    "#,
        flush_entire_data_cache_and_invalidate_tlb = sym flush_entire_data_cache_and_invalidate_tlb,
        prepare_el2_to_el1_transition = sym prepare_el2_to_el1_transition,
        options(noreturn)
    )
}

/// Handles the execution of the Kernel under EL3.
///
/// The implementation triggers a kernel panic since EL3 execution
/// is a broken invariant we cannot recover from.
///
/// # Note
///
/// This function does not make use of the stack.
///
/// # Safety
///
/// This is hardware land. Use cautiously.
#[no_mangle]
extern "C" fn handle_running_under_el3() -> ! {
    // Panics are configured not to unwind the stack.
    panic!("Kernel is running under EL3!")
}

#[inline(always)]
unsafe extern "C" fn prepare_el2_to_el1_transition(ret_addr: u64) {
    let midr = MIDR_EL1.extract();

    // Check if we're running on Cortex-A53 or Cortex-A57 processors and
    // configure implementation-defined registers if that's the case.
    if midr.matches_all(MIDR_EL1::Implementer::Arm)
        && (midr.read(MIDR_EL1::PartNum) == PARTNUM_CORTEX_A57
            || midr.read(MIDR_EL1::PartNum) == PARTNUM_CORTEX_A53)
    {
        // TODO: Proper bitfield.
        //  - CPUACTLR access control = SET
        //  - CPUECTLR access control = SET
        //  - L2CTLR access control = SET
        //  - L2ECTLR access control = SET
        //  - L2ACTLR access control = SET
        ACTLR_EL2.set(0x73);
    }

    // Set EL1 execution state to AArch64.
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    // TODO: Prepare the missing registers.

    // Set up a simulated exception return by masking all interrupts.
    SPSR_EL2.write(SPSR_EL2::M::EL1h + SPSR_EL2::F::Masked + SPSR_EL2::I::Masked);

    // Set up for returning to the supplied address in EL1.
    ELR_EL2.set(ret_addr);
}
