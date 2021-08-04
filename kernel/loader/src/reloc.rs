//! Implementation of the relocation process.

use core::mem;

/// ELF-file specific structures.
pub mod elf {
    pub const DT_NULL: isize = 0;

    pub const DT_RELA: isize = 7;
    pub const DT_RELAENT: isize = 9;
    pub const DT_RELACOUNT: isize = 0x6ffffff9;

    pub const DT_REL: isize = 17;
    pub const DT_RELENT: isize = 19;
    pub const DT_RELCOUNT: isize = 0x6ffff;

    pub const R_AARCH64_RELATIVE: usize = 0x403;

    /// Information about a relocation table from the `.dynamic` section.
    #[derive(Debug, Clone, Default)]
    pub struct RelocationTable {
        /// The offset from the base address to the relocation table.
        pub offset: usize,
        /// The size of each entry inside the relocation table.
        pub entry_size: usize,
        /// The number of entries inside the relocation table
        pub count: usize,
    }

    /// An entry inside the `.dynamic` section of an ELF file.
    #[repr(C)]
    #[derive(Debug, Clone)]
    pub struct Dyn {
        /// The tag of this entry
        pub tag: isize,
        /// It's corresponding value
        pub value: usize,
    }

    /// A relocation entry inside the `.rel` table
    #[repr(C)]
    pub struct Rel {
        pub offset: usize,
        pub info: usize,
    }

    /// A relocation entry inside the `.rela` table
    #[repr(C)]
    pub struct Rela {
        pub offset: usize,
        pub info: usize,
        pub addend: isize,
    }
}

/// The result of a [`relocation`](relocate) operation.
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum RelocationResult {
    /// The relocation was successful
    Ok = 0,
    /// The `DT_xENT` value didn't match the size of [`elf::Rel`] or [`elf::Rela`]
    InvalidEntrySize,
    /// Found a relocation type that is not supported at the moment
    UnsupportedRelocation,
}

/// Apply relocations to the given base address by reading the given `.dynamic` section.
pub unsafe extern "C" fn relocate(base: *mut u8, dynamic: *const u8) -> RelocationResult {
    let mut dynamic = dynamic.cast::<elf::Dyn>();

    // first we need to find the relocation tables from the `.dynamic` section
    let mut rela_offset = None;
    let mut rela_ent = 0;
    let mut rela_count = 0;

    let mut rel_offset = None;
    let mut rel_ent = 0;
    let mut rel_count = 0;

    loop {
        let entry = unsafe { &*dynamic };
        match entry.tag {
            elf::DT_RELA => rela_offset = Some(entry.value),
            elf::DT_RELAENT => rela_ent = entry.value,
            elf::DT_RELACOUNT => rela_count = entry.value,

            elf::DT_REL => rel_offset = Some(entry.value),
            elf::DT_RELENT => rel_ent = entry.value,
            elf::DT_RELCOUNT => rel_count = entry.value,

            elf::DT_NULL => break,
            _ => {}
        }

        dynamic = unsafe { dynamic.add(1) };
    }

    // perform relocations from the `.rela` table
    if let Some(rela_offset) = rela_offset {
        if rela_ent != mem::size_of::<elf::Rela>() {
            return RelocationResult::InvalidEntrySize;
        }

        let table = unsafe { base.add(rela_offset).cast::<elf::Rela>() };
        for idx in 0..rela_count {
            let entry = unsafe { &*table.add(idx) };

            match entry.info & 0xFFFF_FFFF {
                elf::R_AARCH64_RELATIVE => unsafe {
                    let value = base.offset(entry.addend) as usize;
                    base.add(entry.offset).cast::<usize>().write(value);
                },
                _ => return RelocationResult::UnsupportedRelocation,
            }
        }
    }

    // perform relocations from the `.rel` table
    if let Some(rel_offset) = rel_offset {
        if rel_ent != mem::size_of::<elf::Rel>() {
            return RelocationResult::InvalidEntrySize;
        }

        let table = unsafe { base.add(rel_offset).cast::<elf::Rel>() };
        for idx in 0..rel_count {
            let entry = unsafe { &*table.add(idx) };

            match entry.info & 0xFFFF_FFFF {
                elf::R_AARCH64_RELATIVE => unsafe {
                    let ptr = base.add(entry.offset).cast::<usize>();
                    *ptr += base as usize;
                },
                _ => return RelocationResult::UnsupportedRelocation,
            }
        }
    }

    RelocationResult::Ok
}
