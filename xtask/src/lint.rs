//! Implementation of the `lint` action in the build system.

use anyhow::Result;
use xshell::cmd;

use crate::{package::Package, rustc};

/// Lints a given package in the Saturnus source tree.
///
/// This involves running `cargo clippy` and `cargo fmt` for a given package.
///
/// The `check` argument may optionally be used to make `rustfmt` check your
/// code formatting and produce an error on unformatted code.
pub fn lint(pkg: Package, check: bool) -> Result<()> {
    let _cwd = xshell::pushd(rustc::root_dir())?;

    let Package {
        cargo_name, target, ..
    } = pkg;

    // Scan for code smells using cargo clippy.
    cmd!(
        "cargo clippy
            -p {cargo_name}
            --target {target}
            -Zbuild-std=core,alloc,compiler_builtins"
    )
    .run()?;

    // Reformat and eventually check using rustfmt.
    let check_arg = if check { &["--", "--check"][..] } else { &[] };
    cmd!(
        "cargo fmt
            -p {cargo_name}
            {check_arg...}"
    )
    .run()?;

    Ok(())
}
