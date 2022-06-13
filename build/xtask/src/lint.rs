//! Implementation of the build system action `lint`.

use anyhow::Result;
use xshell::{cmd, Shell};

use crate::{package::Package, rustc, target::Target};

/// Lints a given package in the Saturnus source tree.
///
/// This involves running `cargo clippy` and `cargo fmt` for a
/// given package.
///
/// The `check` argument may be used to make rustfmt validate the
/// current code formatting.
pub fn lint(sh: &Shell, pkg: &Package, target: &Target, check: bool) -> Result<()> {
    let _cwd = sh.push_dir(rustc::project_root());

    let cargo_name = pkg.cargo_name;
    let target_json = target.target_json;

    // Scan for code smells using cargo clippy.
    cmd!(
        sh,
        "cargo clippy
            -p {cargo_name}
            --target {target_json}
            -Zbuild-std=core,alloc,compiler_builtins"
    )
    .run()?;

    // Reformat and eventually check using rustfmt.
    let check_arg = if check { &["--", "--check"][..] } else { &[] };
    cmd!(
        sh,
        "cargo fmt
            -p {cargo_name}
            {check_arg...}"
    )
    .run()?;

    Ok(())
}
