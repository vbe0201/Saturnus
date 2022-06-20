//! Early `r0` initialization code for the Kernel.
//!
//! This is inspired by `crt0` and provides the routines needed
//! to do early kernel initialization after it is started.

// WARNING: The soundness of the code in this module relies entirely
// on avoiding the use of stack memory which is not initialized at
// this point. Thus, proceed with caution when making changes below.

use core::arch::asm;

use cortex_a::{
    asm::barrier::{dsb, isb, SY},
    registers::SCTLR_EL1,
};
use tock_registers::interfaces::ReadWriteable;

pub mod cache;
pub mod el;

/// Disables the MMU and instruction/data caches.
///
/// This will invalidate the instruction cache before fully
/// disabling them along with the MMU.
///
/// # Note
///
/// This function does not make use of the stack in any form.
///
/// # Safety
///
/// This is hardware land. Use cautiously.
#[no_mangle]
pub unsafe extern "C" fn disable_mmu_and_caches() {
    // Invalidate the instruction cache.
    asm!("ic ialluis", options(nostack));
    dsb(SY);
    isb(SY);

    // Disables the MMU and instruction/data caches.
    SCTLR_EL1
        .modify(SCTLR_EL1::M::Disable + SCTLR_EL1::C::NonCacheable + SCTLR_EL1::I::NonCacheable);

    // Ensure instruction coherency.
    dsb(SY);
    isb(SY);
}
