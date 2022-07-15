//! Implementation of the build system action `run`.

use std::path::PathBuf;

use anyhow::Result;
use xshell::{cmd, Shell};

use crate::{
    build,
    target::{qemu_parts, Target},
};

/// Builds the full Saturnus kernel image and subsequently tries
/// to emulate it in QEMU if the target supports it.
///
/// The building step is delegated to [`build::build_kernel`].
pub fn run(sh: &Shell, target: &Target, release: bool) -> Result<()> {
    let (system, extra_flags) = qemu_parts(target)?;
    let raw = build::build_kernel(sh, target, release)?;

    run_qemu(sh, raw, system, extra_flags)
}

fn run_qemu(sh: &Shell, kernel: PathBuf, system: &str, extra_flags: &[&str]) -> Result<()> {
    cmd!(
        sh,
        "qemu-system-{system}
            {extra_flags...}
            -machine virt
            -nographic
            -semihosting-config enable=on,target=native
            -kernel {kernel}"
    )
    .run()?;

    Ok(())
}
