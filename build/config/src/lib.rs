//! Static, target-specific configuration of Kernel build
//! options.
//!
//! The implementation of the Kernel should always access
//! [`CURRENT_BUILD`] to learn its configuration, which is
//! one of the target [`Config`]s exposed by the crate.

#![no_std]

/// Static Kernel configuration settings used during build.
///
/// Defines target-specific settings that the Kernel should
/// respect during build.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// The fixed page size that is used.
    pub page_size: usize,
}

/// The configuration that covers the current build.
///
/// This respects the target architecture that is being compiled
/// for along with the Cargo features enabled for this crate.
///
/// May be [`None`] when building for an unsupported platform.
#[allow(unreachable_patterns)]
pub const CURRENT_BUILD: Option<Config> = match () {
    #[cfg(all(target_arch = "aarch64", feature = "qemu"))]
    () => Some(AARCH64_QEMU),

    () => None,
};

/// The build configuration for the `aarch64-qemu` target.
pub const AARCH64_QEMU: Config = Config { page_size: 0x1000 };
