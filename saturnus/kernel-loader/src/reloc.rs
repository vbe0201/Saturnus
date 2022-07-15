use core::slice;

use goblin::elf64::{
    dynamic::{self, Dyn, DynamicInfo},
    program_header::{self as ph, ProgramHeader},
    reloc::{self, Rel, Rela},
};

// This does not correspond to real program headers. We just need it
// for `goblin` to translate addresses. A real base must be supplied
// or we will end up producing broken relocations with the result.
fn make_phdr_for_address_translation(base: usize) -> ProgramHeader {
    ProgramHeader {
        p_type: ph::PT_LOAD,
        p_flags: ph::PF_R | ph::PF_W | ph::PF_X,
        p_offset: base as u64,
        p_filesz: u64::MAX,
        p_memsz: u64::MAX,
        ..Default::default()
    }
}

unsafe fn count_dynamic_entries<'d>(section_start: *const u8) -> &'d [Dyn] {
    let ptr = section_start.cast::<Dyn>();

    // Count all entries in the dynamic section until we hit the DT_NULL tag.
    // SAFETY: We always have at the very least one DT_NULL entry.
    let mut idx = 0;
    while (*ptr.offset(idx)).d_tag != dynamic::DT_NULL {
        idx += 1;
    }

    // SAFETY: The caller ensures the pointer meets the requirements.
    // The `len` is the checked value we just computed above.
    slice::from_raw_parts(ptr, idx as usize)
}

/// Applies relocations to all entries of the given `.dynamic`
/// section, using `base` as the starting point.
///
/// # Safety
///
/// - `base` must point to the very start of the binary in memory.
/// - `dynamic` must point to the address provided by the
///   `_DYNAMIC` linker symbol.
///   - This ensures correct alignment and values to access.
///   - The values have sufficient lifetime.
///   - The memory will not be mutated in this scope.
///
/// # Panics
///
/// Panics when any relocations other than architecture-relative
/// ones are encountered.
#[no_mangle]
pub unsafe extern "C" fn apply_relocations(base: *mut u8, dynamic: *const u8) {
    debug_assert!(dynamic > base as *const u8);

    // Extract all the relocations from the `.dynamic` section.
    let dynamic_info = {
        let dynamic = count_dynamic_entries(dynamic);
        let phdrs = &[make_phdr_for_address_translation(base.addr())];

        DynamicInfo::new(dynamic, phdrs)
    };

    // Apply all `Rel` relocations.
    for i in 0..dynamic_info.relcount {
        // Get a handle to the rel segment.
        let rel = &*dynamic
            .with_addr(dynamic_info.rel)
            .add(dynamic_info.relent as usize * i)
            .cast::<Rel>();

        // Apply the relocation.
        match reloc::r_type(rel.r_info) {
            #[cfg(target_arch = "aarch64")]
            reloc::R_AARCH64_RELATIVE => {
                let ptr = base.add(rel.r_offset as usize).cast::<usize>();
                ptr.write(ptr.read() + base.addr());
            }

            _ => panic!("Unsupported relocation type encountered"),
        }
    }

    // Apply all `Rela` relocations.
    for i in 0..dynamic_info.relacount {
        // Get a handle to the rela segment.
        let rela = &*dynamic
            .with_addr(dynamic_info.rela)
            .add(dynamic_info.relaent as usize * i)
            .cast::<Rela>();

        // Apply the relocation.
        match reloc::r_type(rela.r_info) {
            #[cfg(target_arch = "aarch64")]
            reloc::R_AARCH64_RELATIVE => {
                let value = base.offset(rela.r_addend as isize) as usize;
                base.add(rela.r_offset as usize)
                    .cast::<usize>()
                    .write(value);
            }

            _ => panic!("Unsupported relocation type encountered"),
        }
    }
}
