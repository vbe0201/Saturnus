//! Implementation of the build system action `check`.

use anyhow::Result;
use xshell::{cmd, Shell};

use crate::{package::Package, rustc, target::Target};

pub fn check(sh: &Shell, pkg: &Package, target: &Target, release: bool) -> Result<()> {
    let _cwd = sh.push_dir(rustc::project_root());

    let release_arg = if release { &["--release"][..] } else { &[] };
    let cargo_name = pkg.cargo_name;
    let target_json = target.target_json;
    let features = &["--no-default-features", "--features", target.board];

    // Check the requested package using cargo.
    cmd!(
        sh,
        "cargo check
            {release_arg...}
            -p {cargo_name}
            --target {target_json}
            {features...}
            -Zbuild-std=core,alloc,compiler_builtins"
    )
    .run()?;

    Ok(())
}
