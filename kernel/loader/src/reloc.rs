//! Implementation of the relocation process.

use goblin::elf64::{
    dynamic::{self, Dyn, DynamicInfo},
    program_header::ProgramHeader,
};

/// Pseudo list of all program headers. This list does not correspond to the real program headers,
/// and instead is needed because `goblin` needs a list of program headers for translating
/// addresses.
const PHDRS: &[ProgramHeader] = &[ProgramHeader {
    p_vaddr: 0,
    p_offset: 0,
    p_memsz: u64::MAX,
    p_type: 0,
    p_flags: 0,
    p_paddr: 0,
    p_filesz: 0,
    p_align: 0,
}];

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
pub unsafe extern "C" fn relocate(base: *mut u8, dynamic: *const u8) -> RelocationResult {
    // get all the information from the `.dynamic` section
    let dyn_list = unsafe {
        let dynp = dynamic.cast::<Dyn>();
        let mut idx = 0;
        while u64::from((*dynp.offset(idx)).d_tag) != dynamic::DT_NULL {
            idx += 1;
        }
        core::slice::from_raw_parts(dynp, idx as usize)
    };
    let info = DynamicInfo::new(dyn_list, PHDRS);

    RelocationResult::Ok
}
