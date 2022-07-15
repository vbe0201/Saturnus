//! Saturnus build target definitions for miscellaneous
//! architectures and boards.

use anyhow::{bail, Result};

/// Defines a Saturnus build target for a specific platform
/// and board.
#[derive(Clone, Copy, Debug)]
pub struct Target {
    /// The name of the target.
    pub name: &'static str,
    /// The targeted architecture.
    pub arch: &'static str,
    /// The corresponding target definition file.
    pub target_json: &'static str,
    /// The board we're building for.
    pub board: &'static str,
    /// The build configuration for this target.
    pub config: &'static saturnus_config::Config,
}

const TARGETS: &[Target] = &[
    Target {
        name: "aarch64-qemu",
        arch: "aarch64",
        target_json: "build/targets/aarch64-saturnus-qemu.json",
        board: "qemu",
        config: &saturnus_config::AARCH64_QEMU,
    },
    // TODO: Make this a thing.
    //Target {
    //    name: "aarch64-nintendo-nx",
    //    arch: "aarch64",
    //    target_json: "aarch64-saturnus-nintendo_nx.json",
    //    board: "nx",
    //    config: &saturnus_config::AARCH64_NINTENDO_NX,
    //},
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
    match target.name {
        "aarch64-qemu" => Ok(("aarch64", &["-cpu", "cortex-a57"])),
        _ => bail!("target does not support QEMU emulation"),
    }
}
