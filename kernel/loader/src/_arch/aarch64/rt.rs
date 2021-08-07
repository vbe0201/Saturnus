// This is an architecture-specific module that is made available through the
// path attribute. See the generic module, [`crate::rt`], for orientation.

use core::mem::MaybeUninit;
use cortex_a::registers::{MIDR_EL1, TPIDR_EL1};
use tock_registers::interfaces::{Readable, Writeable};

/// The architecture-specific relocation type for AArch64.
pub use goblin::elf64::reloc::R_AARCH64_RELATIVE as R_ARCHITECTURE_SPECIFIC;

/// Implementer ID of an ARM limited processor.
pub const ARM_LIMITED_ID: u8 = 0x41;

/// Identifier for the Cortex-A57 architecture.
pub const ARCH_CORTEX_A57: u64 = 0xD07;

/// Identifier for the Cortex-A53 architecture.
pub const ARCH_CORTEX_A53: u64 = 0xD03;

#[derive(Debug, Clone)]
#[repr(C)]
struct RegisterState {
    /// Registers X19-X30
    regs: [usize; 12],
    /// stack pointer
    sp: usize,
    /// zero register
    ///
    /// This entry only exists so we have an even amount of register for `stp`
    xzr: usize,
}

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn save_register_state(state: &mut MaybeUninit<RegisterState>) {
    asm!("
        // Write the pointer to the register state into `TPIDR_EL1`
        msr TPIDR_EL1, {0}

        // Save registers into the regsiter state
        mov x1, sp
        stp x19, x20, [{0}], #0x10
        stp x21, x22, [{0}], #0x10
        stp x23, x24, [{0}], #0x10
        stp x25, x26, [{0}], #0x10
        stp x27, x28, [{0}], #0x10
        stp x29, x30, [{0}], #0x10
        stp x1,  xzr, [{0}], #0x10
        ", inout(reg) state => _, out("x1") _,
    )
}

pub unsafe fn arch_specific_setup() {
    // save register state to TPIDR_EL1
    let mut register_state = MaybeUninit::<RegisterState>::uninit();
    unsafe {
        save_register_state(&mut register_state);
    }

    // check which CPU we are running, and configure CPUECTLR, CPUACTLR appropriately
    let manufacture_id = MIDR_EL1.get();
    let implementer = (manufacture_id >> 24) as u8;

    if implementer == ARM_LIMITED_ID {
        // extract value from the manufacture id
        let arch = (manufacture_id >> 4) & 0x0FFF;
        let hw_variant = (manufacture_id >> 20) & 0xF;
        let hw_revision = (manufacture_id >> 20) & 0xF;

        // FIXME: Replace all of this with `tock-registers`
        // abstractions in our own aarch64 crate

        let ctlr_values = match arch {
            // Cortex-A57 specific values
            ARCH_CORTEX_A57 => {
                // non-cacheable load forwarding enabled
                let mut cpuactlr_value = 0x1000000u64;

                // - enable the processor to receive instruction cache
                //   and TLB maintenance operations broadcast from other processors
                //   in the cluster
                // - set the L2 load/store data prefetch distance to 8 requests
                // - set the L2 instruction fetch prefetch distance to 3 requests.
                let cpuectlr_value = 0x1B00000040u64;

                // if supported, disable load-pass DMB.
                if hw_variant == 0 || (hw_variant == 1 && hw_revision <= 1) {
                    cpuactlr_value |= 0x800000000000000;
                }

                Some((cpuactlr_value, cpuectlr_value))
            }
            // Cortex-A53 specific values
            ARCH_CORTEX_A53 => {
                // - set L1 data prefetch control to allow 5 outstanding prefetches
                // - enable device split throttle-
                // - set the number of independent data prefetch streams to 2
                // - disable transient and no-read-allocate hints for loads
                // - set write streaming no-allocate threshold so the 128th consecutive
                //   streaming cache line does not allocate in the L1 or L2 cache.
                let mut cpuactlr_value = 0x90CA000u64;

                // enable hardware management of data coherency with other cores in the cluster.
                let cpuectlr_value = 0x40u64;

                // if supported, enable data cache clean as data cache clean/invalidate.
                if hw_variant != 0 || (hw_variant == 0 && hw_revision > 2) {
                    cpuactlr_value |= 0x100000000000;
                }

                Some((cpuactlr_value, cpuectlr_value))
            }
            _ => None,
        };

        if let Some((cpuactlr, cpuectlr)) = ctlr_values {
            unsafe {
                asm!("
                    // Set CPUACTLR_EL1 implementation defined register
                    msr S3_1_C15_C2_0, {cpua_ctlr}

                    // Set CPUECTLR_EL1 implementation defined register,
                    // if it is not equal to the value we want to set it to
                    mrs {tmp}, S3_1_C15_C2_1
                    cmp {tmp}, {cpue_ctlr}
                    b.eq 1f
                    msr S3_1_C15_C2_1, {cpue_ctlr}
                1:
                    ",
                    cpua_ctlr = in(reg) cpuactlr,
                    cpue_ctlr = in(reg) cpuectlr,
                    tmp = out(reg) _
                )
            }
        }
    }

    // verity taht TPIDR_EL1 is still set to the original register state,
    // and clear it afterwards
    assert_eq!(
        TPIDR_EL1.get(),
        &register_state as *const _ as u64,
        "TPIDR_EL1 didn't match after architecture specific setup"
    );
    TPIDR_EL1.set(0);
}
