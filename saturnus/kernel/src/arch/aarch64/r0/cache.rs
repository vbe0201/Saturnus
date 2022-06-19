//! Stackless functions for CPU cache management.

use core::arch::asm;

use cortex_a::{
    asm::barrier::{dsb, isb, SY},
    registers::{CCSIDR_EL1, CLIDR_EL1, CSSELR_EL1, ID_AA64MMFR2_EL1},
};
use tock_registers::interfaces::{Readable, Writeable};

/// Flushes the CPU data cache and invalidates the entire TLB.
///
/// This function insures instruction and data coherency.
///
/// # Note
///
/// This function does not make use of the stack in any form.
///
/// # Safety
///
/// This is hardware land. Use cautiously.
#[no_mangle]
#[optimize(speed)]
pub unsafe extern "C" fn flush_entire_data_cache_and_invalidate_tlb() {
    // Make sure that the full data cache is coherent.
    flush_entire_data_cache_local();
    flush_entire_data_cache_shared();
    flush_entire_data_cache_local();

    // Invalidate the entire TLB.
    asm!("tlbi vmalle1is", options(nostack));
    dsb(SY);
    isb(SY);
}

/// Flushes the entire local CPU data cache.
///
/// This walks thorugh all the available levels of
/// caches and flushes them by Ways/Sets, respectively.
///
/// # Note
///
/// This function does not make use of the stack in any form.
///
/// # Safety
///
/// This is hardware land. Use cautiously.
#[optimize(speed)]
pub unsafe fn flush_entire_data_cache_local() {
    // Flush all the levels of unification in local cache.
    for level in 0..CLIDR_EL1.read(CLIDR_EL1::LoUIS) {
        flush_cache_level(level);
    }

    // Wait for outstanding cache modifications to complete.
    dsb(SY);
}

/// Flushes the entire shared CPU data cache.
///
/// This walks thorugh all the available levels of
/// caches and flushes them by Ways/Sets, respectively.
///
/// # Note
///
/// This function does not make use of the stack in any form.
///
/// # Safety
///
/// This is hardware land. Use cautiously.
#[optimize(speed)]
pub unsafe fn flush_entire_data_cache_shared() {
    let clidr = CLIDR_EL1.extract();

    let levels_of_unification = clidr.read(CLIDR_EL1::LoUIS);
    let levels_of_coherence = clidr.read(CLIDR_EL1::LoC);

    // Flush all the levels of coherence in shared cache.
    for level in levels_of_unification..=levels_of_coherence {
        flush_cache_level(level);
    }

    // Wait for outstanding cache modifications to complete.
    dsb(SY);
}

// SAFETY: `level` must be implemented for the processor.
#[inline(always)]
unsafe fn flush_cache_level(level: u64) {
    debug_assert!(level < 7);

    // Wait for outstanding cache modifications to complete.
    dsb(SY);

    // Commit the level to access onto the Cache Size Selection register.
    CSSELR_EL1.write(CSSELR_EL1::Level.val(level));

    // Wait for the environment modification to take place.
    isb(SY);

    // Read the (now updated) Current Cache Size ID register.
    let (associativity, num_sets, line_size) = read_current_cache_line_details();

    // Determine the shift constants for building CISW bitmasks.
    let way_shift = associativity.leading_zeros();
    let set_shift = line_size + 4;

    // Clear all sets in all ways of the current cache level.
    for way in 0..=associativity {
        for set in 0..=num_sets {
            let cisw_value = (way << way_shift) | (set << set_shift) | (level << 1);
            asm!("dc cisw, {}", in(reg) cisw_value, options(nostack));
        }
    }
}

#[inline(always)]
fn read_current_cache_line_details() -> (u64, u64, u64) {
    let has_feature_ccidx = ID_AA64MMFR2_EL1.is_set(ID_AA64MMFR2_EL1::CCIDX);
    let ccsidr = CCSIDR_EL1.extract();

    let associativity = match has_feature_ccidx {
        true => ccsidr.read(CCSIDR_EL1::AssociativityWithCCIDX),
        false => ccsidr.read(CCSIDR_EL1::AssociativityWithoutCCIDX),
    };
    let num_sets = match has_feature_ccidx {
        true => ccsidr.read(CCSIDR_EL1::NumSetsWithCCIDX),
        false => ccsidr.read(CCSIDR_EL1::NumSetsWithoutCCIDX),
    };
    let line_size = ccsidr.read(CCSIDR_EL1::LineSize);

    (associativity, num_sets, line_size)
}
