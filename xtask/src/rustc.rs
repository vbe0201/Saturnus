use std::path::PathBuf;

use anyhow::Result;
use xshell::cmd;

/// Returns the path to the root of this cargo workspace.
pub fn root_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path
}

/// Get the path to the sysroot of the current rust compiler.
pub fn sysroot() -> Result<String> {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = cmd!("{rustc} --print sysroot").output()?;
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

/// Get the path to the llvm tool with the given name, that is shipped with rust.
pub fn llvm_tool(tool: &str) -> Result<PathBuf> {
    let mut pathbuf = PathBuf::from(sysroot()?);
    pathbuf.push("lib");
    pathbuf.push("rustlib");
    pathbuf.push(rustc_version::version_meta()?.host);
    pathbuf.push("bin");
    pathbuf.push(format!("llvm-{}", tool));

    Ok(pathbuf)
}
