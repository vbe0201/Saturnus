//!

#![feature(ptr_as_uninit, strict_provenance)]
#![no_std]

pub use config::Config;

mod arch;

pub mod addr;
pub mod init;

/// The build configuration for the currently configured target.
pub const BUILD_CONFIG: Config = match config::CURRENT_BUILD {
    Some(config) => config,
    None => panic!("Building libkern with unsupported target configuration"),
};
