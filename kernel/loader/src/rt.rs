use core::{mem, slice};

use goblin::elf64::{
    dynamic::{self, Dyn, DynamicInfo},
    program_header::{self as ph, ProgramHeader},
    reloc::{self, Rel, Rela},
};

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/rt.rs"]
mod arch_rt;

use arch_rt::R_ARCHITECTURE_SPECIFIC;

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
            R_ARCHITECTURE_SPECIFIC => {
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
