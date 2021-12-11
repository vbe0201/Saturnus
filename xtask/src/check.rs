//! Implementation of the `check` action in the build system.

use anyhow::Result;
use xshell::cmd;

use crate::{package::Package, rustc};

pub fn check(pkg: Package, bsp: Option<&str>, release: bool) -> Result<()> {
    let _cwd = xshell::pushd(rustc::root_dir())?;

    // build the kernel using cargo
    let release_arg = if release { &["--release"][..] } else { &[] };
    let Package {
        cargo_name, target, ..
    } = pkg;

    let features = match bsp {
        Some(bsp) => vec![
            "--no-default-features".to_owned(),
            "--features".to_owned(),
            format!("bsp-{}", bsp),
        ],
        None => vec![],
    };

    cmd!(
        "cargo check
            {release_arg...}
            -p {cargo_name}
            --target {target}
            {features...}
            -Zbuild-std=core,alloc,compiler_builtins"
    )
    .run()?;

    Ok(())
}
