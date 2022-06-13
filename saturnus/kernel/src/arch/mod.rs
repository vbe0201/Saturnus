//! Architecture-specific Kernel implementation details.

// Include the implementation details of architecture
// specific code depending on the chosen build target.
// This consists of higher-level Rust abstractions and
// initial bootstrap code hand-written in assembly.
cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;

        // This is called by the `start.s` assembly routines
        // when the kernel is executing under EL3. Since this
        // is a semantic flaw, there is nothing else we can do.
        #[no_mangle]
        extern "C" fn __saturnus_panic_when_in_el3() {
            panic!("Kernel is running under EL3!");
        }

        ::core::arch::global_asm!(include_str!("aarch64/start.s"));
    } else {
        compile_error!("Attempted to build for unsupported target architecture!");
    }
}

// TODO: Assert that a module exported everything we're expecting.
