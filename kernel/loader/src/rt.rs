use core::{
    mem::{self, MaybeUninit},
    slice,
};

use cortex_a::registers::{MIDR_EL1, TPIDR_EL1};
use goblin::elf64::{
    dynamic::{self, Dyn, DynamicInfo},
    program_header::{self as ph, ProgramHeader},
    reloc::{self, Rel, Rela},
};
use tock_registers::interfaces::{Readable, Writeable};

// This does not correspond to real program headers, and instead is needed because
// `goblin` can't translate addresses otherwise. We need to supply a real base here
// or otherwise we will end up producing broken relocations.
#[inline(always)]
fn make_phdr_for_address_translation(base: usize) -> ProgramHeader {
    ProgramHeader {
        p_type: ph::PT_LOAD,
        p_flags: ph::PF_R | ph::PF_W | ph::PF_X,
        p_offset: base as u64,
        p_filesz: u64::MAX,
        p_memsz: u64::MAX,
        ..ProgramHeader::default()
    }
}

unsafe fn count_dynamic_entries<'d>(section_start: *const u8) -> &'d [Dyn] {
    let ptr = section_start.cast::<Dyn>();
    let mut idx = 0;

    // Count all entries in the dynamic section until we hit a DT_NULL tag.
    // FIXME: This is undefined behavior.
    while unsafe { *ptr.offset(idx) }.d_tag != dynamic::DT_NULL {
        idx += 1;
    }

    unsafe { slice::from_raw_parts(ptr, idx as usize) }
}

/// The result of a [`relocation`](relocate) operation.
#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum RelocationResult {
    /// The relocation was successful.
    Ok = 0,
    /// Found a relocation type that is not architecture-relative.
    UnsupportedRelocation,
}

/// Applies relocations to all entries of the given `.dynamic` section, using `base`
/// as the starting point.
///
/// # Safety
///
/// - `base` mut point to the very start of code that got linked into the binary.
/// - `dynamic` must point to the address provided by the `_DYNAMIC` linker symbol.
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn relocate(base: *mut u8, dynamic: *const u8) -> RelocationResult {
    assert!(dynamic > base as *const u8);

    // Extract all relevant information from the `.dynamic` section.
    let dynamic_info = {
        let dynamic = count_dynamic_entries(dynamic);
        let phdrs = &[make_phdr_for_address_translation(base as usize)][..];

        DynamicInfo::new(dynamic, phdrs)
    };

    // Apply all `Rel` relocations.
    for i in 0..dynamic_info.relcount {
        // Get a handle to the rel segment.
        let rel = &*(dynamic_info.rel as *const u8)
            .add(dynamic_info.relent as usize * i)
            .cast::<Rel>();

        // Apply the relocation.
        match reloc::r_type(rel.r_info) {
            reloc::R_AARCH64_RELATIVE => {
                let ptr = base.add(rel.r_offset as usize).cast::<usize>();
                ptr.write(ptr.read() + base as usize);
            }
            _ => return RelocationResult::UnsupportedRelocation,
        }
    }

    // Apply all `Rela` relocations.
    for i in 0..dynamic_info.relacount {
        // Get a handle to the rela segment.
        let rela = &*(dynamic_info.rela as *const u8)
            .add(dynamic_info.relaent as usize * i)
            .cast::<Rela>();

        // Apply the relocation.
        match reloc::r_type(rela.r_info) {
            reloc::R_AARCH64_RELATIVE => {
                let value = base.offset(rela.r_addend as isize) as usize;
                base.add(rela.r_offset as usize)
                    .cast::<usize>()
                    .write(value);
            }
            _ => return RelocationResult::UnsupportedRelocation,
        }
    }

    RelocationResult::Ok
}

/// Uniformly calls all the functions in the `.init_array` segment.
///
/// The `.init_array` functions are called before [`crate::main`].
///
/// # Safety
///
/// The linker must provide pointer-aligned `__init_array_start__` and
/// `__init_array_end__` symbols which span the entire `.init_array` section.
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn call_init_array() {
    let (start, end) = linker_symbol!(
        __init_array_start__ as unsafe extern "C" fn(),
        __init_array_end__ as unsafe extern "C" fn(),
    );

    // Calculate the amount of pointers that the segment holds.
    let len = (end as usize - start as usize) / mem::size_of::<unsafe extern "C" fn()>();

    // Compose a slice of all the function pointers and call them.
    for ptr in slice::from_raw_parts(start, len) {
        ptr();
    }
}

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

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn arch_specific_setup() {
    // save register state to TPIDR_EL1
    let mut register_state = MaybeUninit::<RegisterState>::uninit();
    save_register_state(&mut register_state);

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

    // verity taht TPIDR_EL1 is still set to the original register state,
    // and clear it afterwards
    assert_eq!(
        TPIDR_EL1.get(),
        &register_state as *const _ as u64,
        "TPIDR_EL1 didn't match after architecture specific setup"
    );
    TPIDR_EL1.set(0);
}
