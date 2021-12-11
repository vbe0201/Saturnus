//! Implementation of the `run` action in the build system.

use std::path::PathBuf;

use anyhow::Result;
use xshell::cmd;

use crate::build;

/// Emulates a supplied kernel binary in QEMU for debugging.
///
/// This emulates an AArch64 host running off a Cortex-A 57 CPU which
/// corresponds to the native Nintendo Switch system.
pub fn run_qemu_aarch64(kernel: PathBuf) -> Result<()> {
    cmd!(
        "qemu-system-aarch64
            -cpu cortex-a57
            -machine virt
            -nographic
            -semihosting-config enable=on,target=native
            -kernel {kernel}"
    )
    .run()?;
    Ok(())
}

/// Emulates the kernel binary at the given path using [`run_qemu_aarch64`].
pub fn run(kernel: PathBuf) -> Result<()> {
    let raw = build::generate_raw_binary(kernel)?;
    run_qemu_aarch64(raw)
}
