//! Implementation of the relocation process.

use core::slice;

use goblin::elf64::{
    dynamic::{self, Dyn, DynamicInfo},
    program_header::{self, ProgramHeader},
    reloc::{self, Rel, Rela},
};

// This does not correspond to real program headers, and instead is needed because `goblin`
// can't translate addresses otherwise. We need to supply a real base here or otherwise we
// will end up producing broken relocations.
#[inline(always)]
fn make_phdr_for_address_translation(base: u64) -> ProgramHeader {
    ProgramHeader {
        p_type: program_header::PT_LOAD,
        p_flags: program_header::PF_R | program_header::PF_W | program_header::PF_X,
        p_offset: base,
        p_vaddr: 0,
        p_paddr: 0,
        p_filesz: u64::MAX,
        p_memsz: u64::MAX,
        p_align: 0,
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
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum RelocationResult {
    /// The relocation was successful
    Ok = 0,
    /// Found a relocation type that is not supported at the moment
    UnsupportedRelocation,
}

/// Apply relocations to the given base address by reading the given `.dynamic` section.
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn relocate(base: *mut u8, dynamic: *const u8) -> RelocationResult {
    // Extract all relevant information from the `.dynamic` section.
    let dynamic_info = {
        let dynamic = count_dynamic_entries(dynamic);
        let phdrs = &[make_phdr_for_address_translation(base as usize as u64)][..];

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
