//! Implementation of the `llvm` action in the build system.

use anyhow::Result;
use xshell::cmd;

use crate::{build, package::Package, rustc};

/// Runs a chosen LLVM tool with its respective argument on a given package
/// in the Saturnus source tree.
pub fn llvm(
    pkg: Package,
    bsp: Option<&str>,
    release: bool,
    tool: &str,
    args: &[String],
) -> Result<()> {
    let kernel = build::build(pkg, bsp, release)?;

    let tool = rustc::llvm_tool(tool)?;
    cmd!("{tool} {kernel} {args...}").echo_cmd(false).run()?;

    Ok(())
}
