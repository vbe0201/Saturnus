use std::path::PathBuf;

use anyhow::Result;
use xshell::cmd;

/// Run the given binary using `qemu-system-aarch64`
pub fn run_qemu_aarch64(binary: PathBuf) -> Result<()> {
    cmd!(
        "qemu-system-aarch64
            -cpu cortex-a57
            -machine virt
            -nographic
            -semihosting-config enable=on,target=native
            -kernel {binary}"
    )
    .run()?;
    Ok(())
}
