// This is an architecture-specific module that is made available through the
// path attribute. See the generic module, [`crate::rt`], for orientation.

/// The architecture-specific relocation type for AArch64.
pub use goblin::elf64::reloc::R_AARCH64_RELATIVE as R_ARCHITECTURE_SPECIFIC;
