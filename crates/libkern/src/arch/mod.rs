//! Architecture-specific implementation details.

cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else {
        compile_error!("Attempted to build for unsupported target architecture!");
    }
}
