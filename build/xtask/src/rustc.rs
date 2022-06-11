use std::path::PathBuf;

use anyhow::Result;
use xshell::{cmd, Shell};

/// Returns the path to the root of this cargo workspace.
pub fn project_root() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path
}

/// Gets the path to the sysroot of the currently used rustc.
pub fn sysroot(sh: &Shell) -> Result<String> {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = cmd!(sh, "{rustc} --print sysroot").output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

/// Gets the path to an LLVM binutil of the given name.
///
/// NOTE: binutils are expected to be shipped with Rust itself.
pub fn llvm_binutil(sh: &Shell, name: &str) -> Result<PathBuf> {
    let mut pathbuf = PathBuf::from(sysroot(sh)?);

    pathbuf.push("lib");
    pathbuf.push("rustlib");
    pathbuf.push(rustc_version::version_meta()?.host);
    pathbuf.push("bin");
    pathbuf.push(format!("llvm-{name}"));

    Ok(pathbuf)
}
