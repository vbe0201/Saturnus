// This module is responsible for re-exporting and exposing architecture-specific
// implementations of exception handling code depending on the Rust build target.

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/exception.rs"]
mod arch_exception;

/// Configures the exception vector and initializes the exception handling.
///
/// # Safety
///
/// - The safety constraints of the architecture-specific implementation of
/// `setup_exception_vector` must be met.
pub unsafe extern "C" fn setup_exception_vector() {
    unsafe { arch_exception::setup_exception_vector() }
}
