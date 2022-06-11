//! Saturnus build target definitions for miscellaneous
//! architectures and boards.

use anyhow::{bail, Result};

/// Defines a Saturnus build target for a specific platform
/// and board.
#[derive(Clone, Copy, Debug)]
pub struct Target {
    /// The name of the target.
    pub name: &'static str,
    /// The corresponding LLVM target triple.
    pub llvm_triple: &'static str,
    /// The board we're building for.
    pub board: &'static str,
}

const TARGETS: &[Target] = &[
    // TODO: Populate this.
];

/// Attempts to find a [`Target`] by name.
pub fn find_target(name: &str) -> Option<&'static Target> {
    TARGETS.iter().find(|t| t.name.eq_ignore_ascii_case(name))
}

/// Returns an iterator over all available [`Target`]s.
pub fn all_targets() -> impl Iterator<Item = &'static Target> {
    TARGETS.iter()
}

/// Gets the QEMU parts for emulation of a given [`Target`],
/// if supported.
pub fn qemu_parts(target: &Target) -> Result<(&'static str, &'static [&'static str])> {
    match target {
        // TODO: Populate this.
        _ => bail!("target does not support QEMU emulation"),
    }
}
